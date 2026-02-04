use serde::{Serialize, de::DeserializeOwned};
use tauri::{AppHandle, Listener};
use tauri_specta::Event;
use tokio::{sync::oneshot, time};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("frontend timeout")]
    FrontendTimeout,

    #[error("no response from frontend")]
    FrontendNoResponse,

    #[error("oneshot recv error")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        let (source, reason) = match value {
            Error::FrontendTimeout => ("ipc", "frontend_timeout"),
            Error::FrontendNoResponse |
            Error::RecvError(_) |
            Error::Json(_) |
            Error::Tauri(_) => ("ipc", "frontend_error"),
        };
        Self {
            source,
            reason
        }
    }
}

#[derive(Debug, Clone, Serialize, specta::Type, Event)]
pub struct FrontendRequest {
    pub id: String,
    pub payload: FrontendRequestPayload,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum FrontendRequestPayload {
    BiliUuid { now: String },
    BiliTicketHexSign { message: String, key: String },
}

pub async fn request<T>(
    app: &AppHandle,
    payload: FrontendRequestPayload,
) -> Result<T, Error>
where
    T: DeserializeOwned + Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let id = format!("frontend-response:{}", Uuid::new_v4().to_string());

    app.once(&id, move |event| {
        let _ = tx.send(
            serde_json::from_str::<T>(event.payload()).map_err(Error::Json),
        );
    });

    FrontendRequest { id, payload }.emit(app)?;

    let result = time::timeout(time::Duration::from_secs(10), rx)
        .await
        .map_err(|_| Error::FrontendTimeout)?
        .map_err(|_| Error::FrontendNoResponse)??;

    Ok(result)
}
