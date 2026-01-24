use std::sync::{LazyLock, RwLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;

use crate::{
    auth::{AuthManager, Cookie},
    config::Config,
    error::{AppError, AppResult},
    shared::{
        ACCEPT,
        CACHE_CONTROL,
        ORIGIN,
        REFERER,
        USER_AGENT,
    }
};

const COOKIE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([^=;\s]+)=?([^;]*)").expect("Build cookie regex")
});

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid cookie")]
    InvalidCookie,

    #[error("reqwest client not initialized")]
    NotInitialized,

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("from utf8 error: {0}")]
    FromUtf8Error(#[from] std::str::Utf8Error),

    #[error("reqwest header to str error")]
    ToStrError(#[from] reqwest::header::ToStrError),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        Self {
            source: "http".into(),
            reason: value.to_string(),
        }
    }
}

pub struct HttpContext {
    inner: RwLock<Option<reqwest::Client>>,
}

pub enum Method {
    GET,
    POST,
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        match self {
            Self::GET => reqwest::Method::GET,
            Self::POST => reqwest::Method::POST,
        }
    }
}

pub enum Body<'a> {
    Json(serde_json::Value),
    Form(&'a [(&'a str, &'a str)]),
}

pub struct Response {
    pub cookies: Vec<Cookie>,
    pub bytes: bytes::Bytes,
}

impl HttpContext {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn rebuild(&self, config: &Config) -> Result<(), Error> {
        let mut builder = reqwest::ClientBuilder::new();

        if config.proxy.enabled {
            let mut proxy = reqwest::Proxy::all(&config.proxy.address)?;

            let username = config.proxy.username.trim();
            let password = config.proxy.password.trim();

            if !username.is_empty() && !password.is_empty() {
                proxy = proxy.basic_auth(username, password);
            }
            builder = builder.proxy(proxy);
        } else {
            builder = builder.no_proxy();
        };

        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());
        *guard = Some(builder.build()?);
        Ok(())
    }

    pub async fn request(
        &self,
        method: Method,
        url: &str,
        query: Option<&[(&str, &str)]>,
        cookies: Option<String>,
        body: Option<Body<'_>>,
    ) -> Result<Response, Error> {
        use reqwest::header;

        let client = {
            let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
            guard.clone().ok_or(Error::NotInitialized)?
        };

        let mut req = client.request(method.into(), url);
        req = apply_headers(req);

        if let Some(c) = cookies {
            req = req.header("Cookie", c);
        }

        if let Some(q) = query {
            req = req.query(q);
        }

        if let Some(body) = body {
            match body {
                Body::Json(r) => req = req.json(&r),
                Body::Form(r) => req = req.form(&r),
            }
        }

        let resp = req.send().await?;

        Ok(Response {
            cookies: parse_cookies(resp.headers().get_all(header::SET_COOKIE))?,
            bytes: resp.bytes().await?,
        })
    }
}

fn apply_headers(mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    req = req
        .header("Accept", ACCEPT)
        .header("Cache-Control", CACHE_CONTROL)
        .header("Origin", ORIGIN)
        .header("Referer", REFERER)
        .header("User-Agent", USER_AGENT);
    req
}

fn parse_cookies(
    raw: reqwest::header::GetAll<'_, reqwest::header::HeaderValue>,
) -> Result<Vec<Cookie>, Error> {
    let mut out = Vec::new();

    for item in raw.iter() {
        let cookie = item.to_str()?;

        for (idx, cap) in COOKIE_RE.captures_iter(&cookie).enumerate() {
            let name = cap
                .get(1)
                .map(|v| v.as_str().trim().to_string())
                .ok_or(Error::InvalidCookie)?;

            let value = cap
                .get(2)
                .map(|v| v.as_str().trim().to_string())
                .unwrap_or_default();

            if idx == 0 {
                out.push(Cookie { name, value });
                continue;
            }

            // match name.as_str()
        }
    }

    Ok(out)
}

#[derive(Debug, Deserialize, specta::Type)]
pub struct BiliRequest<'a> {
    pub url: &'a str,
    pub method: BiliRequestMethod,
    pub query: Option<Vec<(&'a str, &'a str)>>,
    pub body: Option<BiliRequestBody<'a>>,
    pub response: BiliRequestResponseType,
}

impl Into<Method> for BiliRequestMethod {
    fn into(self) -> Method {
        match self {
            Self::GET => Method::GET,
            Self::POST => Method::POST,
        }
    }
}

#[derive(Debug, Deserialize, specta::Type)]
pub enum BiliRequestMethod {
    GET,
    POST,
}

#[derive(Debug, Deserialize, specta::Type)]
pub enum BiliRequestBody<'a> {
    Json(&'a str),
    Form(Vec<(&'a str, &'a str)>),
}

impl<'a> TryFrom<&'a BiliRequestBody<'a>> for Body<'a> {
    type Error = Error;

    fn try_from(value: &'a BiliRequestBody<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            BiliRequestBody::Json(r) => Self::Json(serde_json::from_str(r)?),
            BiliRequestBody::Form(r) => Self::Form(r),
        })
    }
}

#[derive(Debug, Deserialize, specta::Type)]
pub enum BiliRequestResponseType {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum BiliResponse {
    // stringified
    Json(String),
    Text(String),
}

#[tauri::command]
#[specta::specta]
pub async fn bili_request(
    app: AppHandle,
    req: BiliRequest<'_>,
) -> AppResult<BiliResponse> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let resp = http
        .request(
            req.method.into(),
            req.url,
            req.query.as_deref(),
            Some(auth.cookies()),
            req.body.as_ref().map(|r| r.try_into()).transpose()?,
        )
        .await?;

    // use str::from_utf8 instead of String::from_utf8 to avoid allocating when bytes are not valid UTF-8.
    let inner = str::from_utf8(&resp.bytes)
        .map_err(|e| Error::FromUtf8Error(e))?
        .to_string();

    Ok(match req.response {
        BiliRequestResponseType::Json => BiliResponse::Json(inner),
        BiliRequestResponseType::Text => BiliResponse::Text(inner),
    })
}
