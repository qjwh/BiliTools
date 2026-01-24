use std::{fs, io, path::PathBuf, sync::RwLock};

use base64::{Engine, engine::general_purpose};
use tauri::{AppHandle, Manager};

mod env;
mod exit;
mod model;
mod scan;
mod sms;

pub use model::Cookie;

use crate::{
    error::{AppError, AppResult},
    http::HttpContext,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sms login failed: {0} ({1})")]
    SmsLoginFailed(String, i32),

    #[error("sms login send code failed: {0} ({1})")]
    SmsLoginSendCodeFailed(String, i32),

    #[error("prepare sms login failed: {0} ({1})")]
    PrepareSmsLoginFailed(String, i32),

    #[error("scan login failed: {0} ({1})")]
    ScanLoginFailed(String, i32),

    #[error("prepare scan login failed: {0} ({1})")]
    PrepareScanLoginFailed(String, i32),

    #[error("exit login failed: {0} ({1})")]
    ExitLoginFailed(String, i32),

    #[error("get buvid4 failed: {0} ({1})")]
    GetBuvid4Failed(String, i32),

    #[error("gen web ticket failed: {0} ({1})")]
    GenWebTicketFailed(String, i32),

    #[error("event error: {0}")]
    Event(#[from] crate::event::Error),

    #[error("HTTP client error: {0}")]
    Http(#[from] crate::http::Error),

    #[error("Io error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        Self {
            source: "auth".into(),
            reason: value.to_string(),
        }
    }
}

pub struct AuthManager {
    inner: RwLock<model::AuthSession>,
    path: PathBuf,
}

impl AuthManager {
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: Default::default(),
            path,
        }
    }

    pub fn load(&self) -> Result<(), Error> {
        if !self.path.exists() {
            return Ok(());
        }
        let encoded = fs::read(&self.path)?;
        let bytes = general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());
        *guard = serde_json::from_slice(&bytes)?;
        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<(), Error> {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
        let inner = serde_json::to_string(
            &*guard
        )?;
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(inner);
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, encoded.as_bytes())?;
        Ok(())
    }

    pub fn with_session<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut model::AuthSession) -> R,
    {
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());
        f(&mut guard)
    }

    pub fn csrf_token(&self) -> Option<String> {
        self.inner
            .read()
            .unwrap_or_else(|s| s.into_inner())
            .cookies
            .get("bili_jct")
            .map(|s| s.to_owned())
    }

    pub fn cookies(&self) -> String {
        self.with_session(|s| s.cookies.to_cookie_header())
    }
}

#[tauri::command]
#[specta::specta]
pub async fn ensure_request_env(app: AppHandle) -> AppResult<()> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    env::ensure_base(&auth, &http).await?;
    env::ensure_uuid(&app, &auth).await?;
    env::ensure_bili_ticket(&app, &auth, &http).await?;
    env::ensure_buvid(&auth, &http).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn scan_login_prepare(
    app: AppHandle,
) -> AppResult<scan::ScanLoginPrepareResponseData> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = scan::scan_login_prepare(&auth, &http).await?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn scan_login(
    app: AppHandle,
    qrcode_key: String,
) -> AppResult<scan::ScanLoginStatus> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = scan::scan_login(&auth, &http, &qrcode_key).await?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn sms_login_prepare(
    app: AppHandle,
) -> AppResult<sms::SmsCaptchaResponseData> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = sms::sms_login_prepare(&auth, &http).await?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn sms_login_send_code(
    app: AppHandle,
    cid: &str,
    tel: &str,
    token: &str,
    gt: &str,
    challenge: &str,
) -> AppResult<sms::SmsSendCodeResponseData> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = sms::sms_login_send_code(
        &auth, &http, cid, tel, token, gt, challenge
    ).await?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn sms_login(
    app: AppHandle,
    cid: &str,
    tel: &str,
    code: &str,
    captcha_key: &str,
) -> AppResult<()> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = sms::sms_login(
        &auth, &http, cid, tel, code, captcha_key
    ).await?;
    Ok(data)
}

#[tauri::command]
#[specta::specta]
pub async fn exit_login(app: AppHandle) -> AppResult<()> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    exit::exit_login(&auth, &http).await?;
    Ok(())
}
