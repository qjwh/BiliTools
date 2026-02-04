mod model;

use std::{fs, io, path::PathBuf, sync::Arc};

use arc_swap::ArcSwap;
pub use model::Config;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{
    error::AppError,
    shared::{Locale, Theme, WindowEffect},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("TOML deserialize error: {0}")]
    TomlDe(#[from] toml::de::Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        let (source, reason) = match value {
            Error::Io(_) => ("config", "io_failed"),

            Error::TomlDe(_) |
            Error::TomlSer(_) => ("config", "parse_faild"),
        };
        Self {
            source,
            reason
        }
    }
}

pub struct ConfigManager {
    inner: ArcSwap<Config>,
    path: PathBuf,
}

impl ConfigManager {
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: ArcSwap::from_pointee(Config::default()),
            path,
        }
    }

    pub fn load(&self) -> Result<(), Error> {
        if !self.path.exists() {
            return Ok(());
        }
        let bytes = fs::read(&self.path)?;
        let inner = toml::from_slice(&bytes)?;
        self.inner.store(Arc::new(inner));
        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<(), Error> {
        let inner = toml::to_string_pretty(
            &*self.inner.load()
        )?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, inner.as_bytes())?;
        Ok(())
    }

    pub fn snapshot(&self) -> Arc<Config> {
        self.inner.load_full()
    }

    pub fn with_config<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Config) -> R,
    {
        let guard = self.inner.load();
        f(&*guard)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ResolvedConfig {
    config: Arc<Config>,
    locale: Locale,
    theme: Theme,
    window_effect: Option<WindowEffect>,
}

#[tauri::command]
#[specta::specta]
pub fn resolve_config(app: AppHandle) -> ResolvedConfig {
    let config = app.state::<ConfigManager>();
    let (locale, theme, window_effect) = config.with_config(|c| {
        (
            c.ui.locale.resolve(),
            c.ui.theme.resolve(),
            c.ui.window_effect.resolve(),
        )
    });
    ResolvedConfig {
        config: config.snapshot(),
        locale,
        theme,
        window_effect,
    }
}
