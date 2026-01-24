use serde::{Deserialize, Serialize};

mod ui;
mod proxy;
mod aria2;

pub use ui::UiConfig;
pub use proxy::ProxyConfig;
pub use aria2::Aria2Config;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Config {
    pub ui: UiConfig,
    pub proxy: ProxyConfig,
    pub aria2: Aria2Config
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig::default(),
            proxy: ProxyConfig::default(),
            aria2: Aria2Config::default(),
        }
    }
}
