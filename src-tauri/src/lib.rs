use states::{
    config::load_config, db::DatabaseHelper, qbit::QbitClient, BgmApiClientState, ConfigState,
    QbitClientState, ServeSignalState, TorrentAdapterRegistryState,
};
use tauri::{async_runtime::Mutex, generate_handler, Manager};

mod data;
mod error;
mod events;
mod handlers;
mod states;
mod torrent_adapters;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::missing_panics_doc)]
pub async fn run() {
    let config = load_config().expect("Failed to load config");
    let db_helper_state =
        Mutex::new(DatabaseHelper::try_new().expect("Failed to create database helper"));
    let mut qbit_client = QbitClient::new(config.download_config.clone())
        .await
        .expect("Failed to create qbit client");

    tauri::Builder::default()
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
            // torrent handlers
            handlers::init_search_torrents,
            handlers::get_downloading_torrents_num,
            handlers::add_torrent,
            handlers::get_torrent_stats,
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
            handlers::set_progress
        ])
        .setup(move |app| {
            app.manage(ConfigState::new(config));

            app.manage(db_helper_state);

            app.manage(BgmApiClientState::default());

            app.manage(TorrentAdapterRegistryState::default());

            app.manage(ServeSignalState::default());

            qbit_client.set_app(app.handle().clone());
            app.manage(QbitClientState::new(qbit_client));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
