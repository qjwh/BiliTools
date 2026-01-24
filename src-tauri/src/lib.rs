mod auth;
mod config;
mod error;
mod event;
mod http;
mod init;
mod logger;
mod services;
mod shared;
mod window;

use directories::ProjectDirs;
use specta_typescript::Typescript;
use tauri::{Manager, async_runtime};
use tauri_specta::{Builder, collect_commands, collect_events};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init();
    tracing::info!("BiliTools v{VERSION}");

    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            config::resolve_config,
            init::init,
            auth::ensure_request_env,
            auth::scan_login_prepare,
            auth::scan_login,
            auth::sms_login_prepare,
            auth::sms_login_send_code,
            auth::sms_login,
            auth::exit_login,
            http::bili_request,
        ])
        .events(collect_events![event::rpc::FrontendRequest,]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().header("// @ts-nocheck"),
            "../generated/backend.ts",
        )
        .expect("Failed to export typescript bindings");

    let dirs = ProjectDirs::from("com", "btjawa", "bilitools")
        .expect("Get ProjectDirs");

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            app.manage(init::InitState::new());
            app.manage(config::ConfigManager::new(
                dirs.config_dir().join("config.toml"),
            ));
            app.manage(auth::AuthManager::new(
                dirs.data_dir().join("auth.session"),
            ));
            app.manage(http::HttpContext::new());
            app.manage(services::ServicesManager::new(
                dirs.cache_dir()
            ));

            let init_state = app.state::<init::InitState>();
            let config_manager = app.state::<config::ConfigManager>();
            let auth_manager = app.state::<auth::AuthManager>();
            let http_context = app.state::<http::HttpContext>();

            init_state.result(config_manager.load().map_err(Into::into));
            init_state.result(auth_manager.load().map_err(Into::into));
            
            let config = &*config_manager.snapshot();

            init_state.result(http_context.rebuild(config).map_err(Into::into));

            window::configure(app.handle(), config)?;

            let handle_future = app.handle().clone();
            async_runtime::spawn(async {
                let app = handle_future;

                let init_state = app.state::<init::InitState>();
                let config_manager = app.state::<config::ConfigManager>();
                let services_manager = app.state::<services::ServicesManager>();

                init_state.result(
                    services_manager.run(
                        &app,
                        &*config_manager.snapshot()
                    ).await.map_err(Into::into)
                );
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running BiliTools");
}
