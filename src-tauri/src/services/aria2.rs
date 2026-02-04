use std::path::PathBuf;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tokio::sync::RwLock;

use crate::config::Config;
use crate::error::AppError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tauri sidecar error: {0}")]
    Shell(#[from] tauri_plugin_shell::Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        let (source, reason) = match value {
            Error::Shell(_) => ("aria2", "spawn_failed"),
        };
        Self {
            source,
            reason
        }
    }
}

pub struct Aria2Service {
    cache: PathBuf,
}

impl Aria2Service {
    pub fn new(cache: PathBuf) -> Self {
        Self {
            cache,
        }
    }

    pub async fn run(&self, app: &tauri::AppHandle, cfg: &Config) -> Result<(), Error> {
        let conf_path = self.cache.join("aria2.conf");
        let session_path = self.cache.join("aria2.session");
        Ok(())
    }

    async fn daemon(&self) {
        loop {

        }
    }
}

struct RpcRequest {

}

impl RpcRequest {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
