use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum Aria2Exec {
    Bundled,
    Custom(PathBuf),
}

impl Aria2Exec {
    pub fn resolve(&self) -> PathBuf {
        match self {
            // tauri_plugin_shell automatically applies the `.exe` extension on Windows.
            Self::Bundled => PathBuf::from("aria2c"),
            Self::Custom(r) => r.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Aria2Config {
    exec: Aria2Exec,
    conf: Aria2ConfigConf,
}

impl Default for Aria2Config {
    fn default() -> Self {
        Self {
            exec: Aria2Exec::Bundled,
            conf: Aria2ConfigConf::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub struct Aria2ConfigConf {
    max_concurrent_downloads: String,
    max_overall_download_limit: String,
    max_download_limit: String,
    max_connection_per_server: String,
}

impl Default for Aria2ConfigConf {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: "3".into(),
            max_overall_download_limit: "0".into(),
            max_download_limit: "0".into(),
            max_connection_per_server: "1".into(),
        }
    }
}
