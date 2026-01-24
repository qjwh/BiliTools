pub mod rpc;

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
