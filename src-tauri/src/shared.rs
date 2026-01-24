use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub const ACCEPT: &str = "*/*";
pub const CACHE_CONTROL: &str = "no-cache";
pub const ORIGIN: &str = "https://www.bilibili.com";
pub const REFERER: &str = "https://www.bilibili.com/";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl From<Theme> for tauri::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Light => Self::Light,
            Theme::Dark => Self::Dark,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum WindowEffect {
    /// **macOS 10.10+**
    Titlebar,
    /// **macOS 10.10+**
    Selection,
    /// **macOS 10.11+**
    Menu,
    /// **macOS 10.11+**
    Popover,
    /// **macOS 10.11+**
    Sidebar,
    /// **macOS 10.14+**
    HeaderView,
    /// **macOS 10.14+**
    Sheet,
    /// **macOS 10.14+**
    WindowBackground,
    /// **macOS 10.14+**
    HudWindow,
    /// **macOS 10.14+**
    FullScreenUI,
    /// **macOS 10.14+**
    Tooltip,
    /// **macOS 10.14+**
    ContentBackground,
    /// **macOS 10.14+**
    UnderWindowBackground,
    /// **macOS 10.14+**
    UnderPageBackground,
    /// Mica effect that matches the system dark preference **Windows 11 Only**
    Mica,
    /// Tabbed effect that matches the system dark preference **Windows 11 Only**
    Tabbed,
    /// **Windows 7/10/11(22H1) Only**
    ///
    /// ## Notes
    ///
    /// This effect has bad performance when resizing/dragging the window on Windows 11 build 22621.
    Blur,
    /// **Windows 10/11 Only**
    ///
    /// ## Notes
    ///
    /// This effect has bad performance when resizing/dragging the window on Windows 10 v1903+ and Windows 11 build 22000.
    Acrylic,
}

impl From<WindowEffect> for tauri::window::Effect {
    fn from(value: WindowEffect) -> Self {
        use tauri::window::Effect;
        match value {
            WindowEffect::Titlebar => Effect::Titlebar,
            WindowEffect::Selection => Effect::Selection,
            WindowEffect::Menu => Effect::Menu,
            WindowEffect::Popover => Effect::Popover,
            WindowEffect::Sidebar => Effect::Sidebar,
            WindowEffect::HeaderView => Effect::HeaderView,
            WindowEffect::Sheet => Effect::Sheet,
            WindowEffect::WindowBackground => Effect::WindowBackground,
            WindowEffect::HudWindow => Effect::HudWindow,
            WindowEffect::FullScreenUI => Effect::FullScreenUI,
            WindowEffect::Tooltip => Effect::Tooltip,
            WindowEffect::ContentBackground => Effect::ContentBackground,
            WindowEffect::UnderWindowBackground => {
                Effect::UnderWindowBackground
            }
            WindowEffect::UnderPageBackground => Effect::UnderPageBackground,
            WindowEffect::Mica => Effect::Mica,
            WindowEffect::Tabbed => Effect::Tabbed,
            WindowEffect::Blur => Effect::Blur,
            WindowEffect::Acrylic => Effect::Acrylic,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, specta::Type,
)]
pub enum Locale {
    #[serde(rename = "zh-CN")]
    ZhCN,
    #[serde(rename = "en-US")]
    EnUS,
}

pub fn unix_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
