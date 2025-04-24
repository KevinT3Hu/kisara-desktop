#![feature(error_generic_member_access)]

use error::KisaraResult;
use states::{
    bgm_api::BgmApiClient,
    config::{load_config, KisaraConfig},
    db::DatabaseHelper,
    qbit::QbitClient,
    BgmApiClientState, ConfigState, QbitClientState, ServeSignalState, TorrentAdapterRegistryState,
};
use tauri::{
    async_runtime::Mutex,
    generate_handler,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, RunEvent,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{
    filter::Filtered,
    fmt::{
        format::{DefaultFields, Format, Full},
        time::ChronoLocal,
        Layer,
    },
    reload::Handle,
    Registry,
};

mod data;
mod error;
mod events;
mod handlers;
mod states;
mod torrent_adapters;
mod utils;

pub type TracingReloadHandle = Option<
    Handle<
        Filtered<
            Layer<Registry, DefaultFields, Format<Full, ChronoLocal>, NonBlocking>,
            LevelFilter,
            Registry,
        >,
        Registry,
    >,
>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::missing_panics_doc)]
pub async fn run(reload_handle: TracingReloadHandle) -> KisaraResult<()> {
    let config = load_config()?;
    let db_helper_state = Mutex::new(DatabaseHelper::try_new()?);
    let qbit_client = QbitClient::new(config.download_config.clone()).await?;

    info!("Setting up kisara app with config: {:?}", config);
    let app = setup_app(config, db_helper_state, qbit_client, reload_handle)?;

    info!("Kisara app setup complete, running...");
    app.run(handle_run_event);
    Ok(())
}

fn setup_app(
    config: KisaraConfig,
    db_helper_state: Mutex<DatabaseHelper>,
    mut qbit_client: QbitClient,
    handle: TracingReloadHandle,
) -> tauri::Result<tauri::App> {
    println!("Setting up kisara app with config: {:?}", config);
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![
            // anime handlers
            handlers::current_season_animes,
            handlers::current_season,
            handlers::search_animes,
            handlers::search_suggestions,
            handlers::animes_in_list,
            handlers::add_anime,
            handlers::get_anime,
            handlers::get_episodes,
            handlers::get_history,
            handlers::list_animes,
            handlers::get_last_watched_ep,
            handlers::get_dashboard_summary,
            // torrent handlers
            handlers::init_search_torrents,
            handlers::get_downloading_torrents_num,
            handlers::add_torrent,
            handlers::get_torrent_stats,
            handlers::remove_torrent,
            handlers::torrent_is_present,
            // window handlers
            handlers::get_window_is_maximized,
            handlers::maximize_window,
            handlers::unmaximize_window,
            handlers::minimize_window,
            handlers::close_window,
            handlers::open_url,
            handlers::fullscreen_window,
            handlers::unfullscreen_window,
            // watch handlers
            handlers::parse_torrent_play_info,
            handlers::set_progress,
            // settings handlers
            handlers::get_config,
            handlers::change_locale,
            handlers::set_bangumi_proxy,
            handlers::set_torrents_proxy,
            handlers::select_download_path,
            handlers::set_log_level,
        ])
        .setup(move |app| {
            app.manage(db_helper_state);

            let bgm_api_client = BgmApiClient::new(if config.network_config.bgm_proxy_enabled {
                config.network_config.bgm_proxy.clone()
            } else {
                None
            });
            app.manage(BgmApiClientState::new(bgm_api_client));

            let torrent_adapter_registry = torrent_adapters::TorrentAdapterRegistry::new(
                if config.network_config.torrents_proxy_enabled {
                    config.network_config.torrents_proxy.clone()
                } else {
                    None
                },
            );
            app.manage(TorrentAdapterRegistryState::new(torrent_adapter_registry));

            app.manage(ServeSignalState::default());

            qbit_client.set_app(app.handle().clone());
            app.manage(QbitClientState::new(qbit_client));

            if let Some(ref r) = handle {
                #[allow(clippy::let_underscore_must_use)]
                let _ = r.modify(|filter| {
                    *filter.filter_mut() = config.debug_config.log_level.clone().into();
                });
            }

            app.manage(handle);
            app.manage(ConfigState::new(config));

            let menu_item_quit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&menu_item_quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .expect("This has been set")
                        .clone(),
                )
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "exit" => {
                        info!("Tray icon clicked, exiting");
                        app.exit(0);
                    }
                    "show" => {
                        let window = app
                            .get_webview_window("main")
                            .expect("Failed to get main window");
                        if window.is_visible().unwrap_or_default() {
                            window.hide().expect("Failed to hide window");
                        } else {
                            window.show().expect("Failed to show window");
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .build(tauri::generate_context!())
}

fn handle_run_event(app_handle: &tauri::AppHandle, e: RunEvent) {
    match e {
        RunEvent::Ready => {
            let _app_handle = app_handle.clone();
        }

        RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let app = app_handle
                        .get_webview_window(&label)
                        .expect("Failed to get webview window");
                    app.hide().expect("Failed to hide");
                    api.prevent_close();
                }
            }
        }

        _ => {}
    }
}
