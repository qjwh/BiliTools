use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            address: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}
