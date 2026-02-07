use tauri::{
    WebviewWindowBuilder, utils::config::WindowEffectsConfig,
    webview::PlatformWebview,
};

use super::config::Config;

pub fn configure(
    app: &tauri::AppHandle,
    cfg: &Config,
) -> tauri::Result<tauri::WebviewWindow> {
    let mut builder = WebviewWindowBuilder::new(
        app,
        "main",
        tauri::WebviewUrl::default(),
    )
    .min_inner_size(960.0, 680.0)
    .inner_size(960.0, 680.0)
    .title("BiliTools")
    .theme(Some(cfg.ui.theme.resolve().into()))
    .effects(WindowEffectsConfig {
        effects: cfg
            .ui
            .window_effect
            .resolve()
            .map(|r| r.into())
            .into_iter()
            .collect(),
        ..Default::default()
    })
    .transparent(true);

    #[cfg(target_os = "macos")]
    {
        tracing::info!("{:?}", tauri_plugin_os::version());
        // For Tahoe, the size of traffic lights is 60*14
        // The x offset is (36-14)/2=11, the y offset is (36-14)/2+9=20

        // For the legacies (Sequoia or older), the size of traffic lights is 52*12
        // The x offset is (36-12)/2=12, the y offset is (36-12)/2+4=16
        let (x, y) = if let tauri_plugin_os::Version::Semantic(major, _, _) =
            tauri_plugin_os::version()
        {
            if major >= 26 {
                (11, 20)
            } else {
                (12, 16)
            }
        } else {
            (12, 16)
        };
        tracing::info!(x, y, "");
        builder = builder
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .traffic_light_position(tauri::LogicalPosition::new(x, y))
            .hidden_title(true);
    }

    #[cfg(not(target_os = "macos"))]
    {
        builder = builder.decorations(false);
    }

    let window = builder.build()?;

    window.with_webview(|webview| {
        #[cfg(target_os = "windows")]
        let _ = with_webview_windows(webview);
        #[cfg(target_os = "macos")]
        let _ = with_webview_macos(webview);
        #[cfg(target_os = "linux")]
        let _ = with_webview_linux(webview);
    })?;

    #[cfg(debug_assertions)]
    window.open_devtools();

    Ok(window)
}

#[cfg(target_os = "windows")]
fn with_webview_windows(
    webview: PlatformWebview,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    unsafe {
        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings5;
        use windows::core::Interface;
        let s = webview
            .controller()
            .CoreWebView2()
            .and_then(|c| c.Settings())
            .and_then(|s| s.cast::<ICoreWebView2Settings5>())?;
        #[cfg(not(debug_assertions))]
        s.SetAreBrowserAcceleratorKeysEnabled(false)?;
        s.SetAreDefaultContextMenusEnabled(false)?;
        s.SetIsPasswordAutosaveEnabled(false)?;
        s.SetIsGeneralAutofillEnabled(false)?;
        s.SetIsZoomControlEnabled(false)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn with_webview_macos(
    webview: PlatformWebview,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn with_webview_linux(
    webview: PlatformWebview,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
