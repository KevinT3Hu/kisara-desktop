use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::{
    error::KisaraResult,
    states::{
        bgm_api::BgmApiClient,
        config::{KisaraConfig, LogLevelFilter},
        BgmApiClientState, ConfigState, TorrentAdapterRegistryState,
    },
    torrent_adapters::TorrentAdapterRegistry,
    TracingReloadHandle,
};

#[tauri::command]
pub async fn get_config(config: State<'_, ConfigState>) -> KisaraResult<KisaraConfig> {
    let config = config.lock().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn change_locale(
    config: State<'_, ConfigState>,
    locale: String,
) -> KisaraResult<KisaraConfig> {
    let mut config = config.lock().await;
    config.locale = locale;
    config.write_config()?;
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_bangumi_proxy(
    config: State<'_, ConfigState>,
    proxy: Option<String>,
    client: State<'_, BgmApiClientState>,
    enabled: bool,
) -> KisaraResult<KisaraConfig> {
    let mut config = config.lock().await;

    if let Some(p) = proxy {
        config.network_config.bgm_proxy = Some(p);
    }

    let new_client = BgmApiClient::new(if enabled {
        config.network_config.bgm_proxy.clone()
    } else {
        None
    });
    *client.lock().await = new_client;

    config.network_config.bgm_proxy_enabled = enabled;

    config.write_config()?;
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_torrents_proxy(
    config: State<'_, ConfigState>,
    proxy: Option<String>,
    registry: State<'_, TorrentAdapterRegistryState>,
    enabled: bool,
) -> KisaraResult<KisaraConfig> {
    let mut config = config.lock().await;
    // if proxy in config and config param are both None and enabled is true, do nothing

    if let Some(p) = proxy {
        config.network_config.torrents_proxy = Some(p);
    }

    let new_registry = TorrentAdapterRegistry::new(if enabled {
        config.network_config.torrents_proxy.clone()
    } else {
        None
    });
    *registry.lock().await = new_registry;

    config.network_config.torrents_proxy_enabled = enabled;

    config.write_config()?;
    Ok(config.clone())
}

#[tauri::command]
pub async fn select_download_path(
    app: AppHandle,
    config: State<'_, ConfigState>,
) -> KisaraResult<KisaraConfig> {
    let folder = app.dialog().file().blocking_pick_folder();

    let mut config = config.lock().await;
    if let Some(f) = folder {
        f.into_path()
            .expect("Should work")
            .to_str()
            .expect("Should work")
            .clone_into(&mut config.download_config.download_path);
        config.write_config()?;
    }

    Ok(config.clone())
}

#[tauri::command]
pub async fn set_log_level(
    config: State<'_, ConfigState>,
    level: LogLevelFilter,
    handle: State<'_, TracingReloadHandle>,
) -> KisaraResult<KisaraConfig> {
    let mut config = config.lock().await;
    config.debug_config.log_level = level.clone();
    config.write_config()?;

    if let Some(ref r) = *handle {
        #[allow(clippy::let_underscore_must_use)]
        let _ = r.modify(|filter| {
            *filter.filter_mut() = level.into();
        });
    }

    Ok(config.clone())
}
