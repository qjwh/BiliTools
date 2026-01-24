use serde::{Deserialize, Serialize};

use crate::shared::{Locale, Theme, WindowEffect};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct UiConfig {
    pub locale: LocaleConfig,
    pub theme: ThemeConfig,
    pub window_effect: WindowEffectConfig,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, specta::Type,
)]
pub enum LocaleConfig {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "zh-CN")]
    ZhCN,
    #[serde(rename = "en-US")]
    EnUS,
}

impl LocaleConfig {
    pub fn resolve(&self) -> Locale {
        match self {
            Self::Auto => match sys_locale::get_locale() {
                Some(code) if code.starts_with("en") => Locale::EnUS,
                Some(code) if code.starts_with("zh") => Locale::ZhCN,
                _ => Locale::EnUS,
            },
            Self::ZhCN => Locale::ZhCN,
            Self::EnUS => Locale::EnUS,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum ThemeConfig {
    /// Auto Detect. Config only.
    Auto,
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl ThemeConfig {
    pub fn resolve(&self) -> Theme {
        match self {
            Self::Auto => match dark_light::detect() {
                Ok(dark_light::Mode::Dark) => Theme::Dark,
                _ => Theme::Light,
            },
            Self::Light => Theme::Light,
            Self::Dark => Theme::Dark,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum WindowEffectConfig {
    /// Auto Detect. Config only.
    Auto,
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

impl WindowEffectConfig {
    pub fn resolve(&self) -> Option<WindowEffect> {
        Some(match self {
            Self::Auto => match tauri_plugin_os::platform() {
                "windows" => {
                    if let tauri_plugin_os::Version::Semantic(_, _, patch) =
                        tauri_plugin_os::version()
                    {
                        if patch >= 22000 {
                            WindowEffect::Mica
                        } else if !(18362..=22000).contains(&patch) {
                            WindowEffect::Acrylic
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                "macos" => WindowEffect::Sidebar,
                _ => return None,
            },
            Self::Titlebar => WindowEffect::Titlebar,
            Self::Selection => WindowEffect::Selection,
            Self::Menu => WindowEffect::Menu,
            Self::Popover => WindowEffect::Popover,
            Self::Sidebar => WindowEffect::Sidebar,
            Self::HeaderView => WindowEffect::HeaderView,
            Self::Sheet => WindowEffect::Sheet,
            Self::WindowBackground => WindowEffect::WindowBackground,
            Self::HudWindow => WindowEffect::HudWindow,
            Self::FullScreenUI => WindowEffect::FullScreenUI,
            Self::Tooltip => WindowEffect::Tooltip,
            Self::ContentBackground => WindowEffect::ContentBackground,
            Self::UnderWindowBackground => WindowEffect::UnderWindowBackground,
            Self::UnderPageBackground => WindowEffect::UnderPageBackground,
            Self::Mica => WindowEffect::Mica,
            Self::Tabbed => WindowEffect::Tabbed,
            Self::Blur => WindowEffect::Blur,
            Self::Acrylic => WindowEffect::Acrylic,
        })
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            locale: LocaleConfig::Auto,
            theme: ThemeConfig::Auto,
            window_effect: WindowEffectConfig::Auto,
        }
    }
}
