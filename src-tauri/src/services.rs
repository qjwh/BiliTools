use std::path::Path;

use crate::{config::Config, error::AppError};

pub mod aria2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("aria2 error: {0}")]
    Aria2(#[from] aria2::Error)
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        Self {
            source: "services".into(),
            reason: value.to_string(),
        }
    }
}

pub struct ServicesManager {
    pub aria2: aria2::Aria2Service,
}

impl ServicesManager {
    pub fn new(cache: &Path) -> Self {
        Self {
            aria2: aria2::Aria2Service::new(
                cache.join("aria2")
            ),
        }
    }

    pub async fn run(&self, app: &tauri::AppHandle, config: &Config) -> Result<(), Error> {
        self.aria2.run(app, config).await?;
        Ok(())
    }
}
