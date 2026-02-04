use std::{fs, io, path::PathBuf, sync::RwLock};

use base64::{Engine, engine::general_purpose};
use tauri::{AppHandle, Manager};

mod env;
mod exit;
mod model;
mod scan;

pub use model::Cookie;

use crate::{
    error::{AppError, AppResult},
    http::HttpContext,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        let (source, reason) = match value {
            Error::Io(_) => ("auth", "io_failed"),

            Error::Base64Decode(_) | 
            Error::Json(_) => ("auth", "parse_failed")
        };
        Self {
            source,
            reason,
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
pub async fn scan_login_gen_qrcode(
    app: AppHandle,
) -> AppResult<scan::ScanLoginPrepareResponseData> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    let data = scan::scan_login_gen_qrcode(&auth, &http).await?;
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
pub async fn exit_login(app: AppHandle) -> AppResult<()> {
    let auth = app.state::<AuthManager>();
    let http = app.state::<HttpContext>();
    exit::exit_login(&auth, &http).await?;
    Ok(())
}
