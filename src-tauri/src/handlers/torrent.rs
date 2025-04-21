use std::collections::HashMap;

use serde::Serialize;
use tauri::State;

use crate::{
    error::KisaraResult,
    states::{
        qbit::ManagedTorrentInfo, DatabaseHelperState, QbitClientState, TorrentAdapterRegistryState,
    },
    torrent_adapters::TorrentInfo,
};

#[tauri::command]
pub async fn init_search_torrents(
    ep_id: i32,
    registry: State<'_, TorrentAdapterRegistryState>,
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<HashMap<String, Vec<TorrentInfo>>> {
    let db_helper = db_helper.lock().await;
    let (anime, ep) = db_helper.get_anime_and_ep_with_ep_id(ep_id).await?;
    let registry = registry.lock().await;
    let results = registry.init_search(&ep, &anime).await?;
    Ok(results)
}

#[tauri::command]
pub async fn get_downloading_torrents_num(
    qbit_client: State<'_, QbitClientState>,
) -> KisaraResult<u32> {
    let qbit_client = qbit_client.lock().await;
    Ok(qbit_client.get_downloading_torrents())
}

#[tauri::command]
pub async fn add_torrent(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    magnet: String,
    ep_id: i32,
) -> KisaraResult<()> {
    let qbit_client = qbit_client.lock().await;
    let torrent_id = qbit_client.add_torrent(&magnet).await?;
    let db_helper = db_helper.lock().await;
    db_helper.set_episode_torrent_id(ep_id, torrent_id).await?;
    Ok(())
}

#[derive(Serialize)]
pub struct TorrentStat {
    pub ep_display: String,
    pub info: ManagedTorrentInfo,
    pub torrent_id: String,
}

#[tauri::command]
pub async fn get_torrent_stats(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<TorrentStat>> {
    let qbit_client = qbit_client.lock().await;
    let stats = qbit_client.get_torrent_stats();
    let mut stats = stats.into_iter().collect::<Vec<_>>();
    stats.sort_by(|a, b| a.0.cmp(&b.0));

    let db_helper = db_helper.lock().await;
    let mut torrent_stats = Vec::new();
    for (id, torrent) in stats {
        let ep = db_helper.get_ep_with_torrent_id(id.to_string()).await.ok();
        if let Some(ep) = ep {
            let anime = db_helper.get_anime_with_ep_id(ep.id).await?;
            let ep_display = format!("{} - E{}", anime.name_cn, ep.ep.unwrap_or(ep.sort));
            torrent_stats.push(TorrentStat {
                ep_display,
                info: ManagedTorrentInfo {
                    name: torrent.name.clone(),
                    stats: torrent.stats,
                },
                torrent_id: id.to_string(),
            });
        }
    }
    Ok(torrent_stats)
}

#[tauri::command]
pub async fn remove_torrent(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    torrent_id: String,
) -> KisaraResult<()> {
    let qbit_client = qbit_client.lock().await;
    qbit_client.remove_torrent(&torrent_id).await?;
    let db_helper = db_helper.lock().await;
    db_helper.remove_torrent(torrent_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn torrent_is_present(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    ep_id: i32,
) -> KisaraResult<Option<String>> {
    let db_helper = db_helper.lock().await;
    let torrent_id = db_helper.get_torrent_id_with_ep_id(ep_id).await?;
    if let Some(torrent_id) = torrent_id {
        let qbit_client = qbit_client.lock().await;
        if qbit_client.torrent_exists(&torrent_id)? {
            return Ok(Some(torrent_id));
        }
        db_helper.remove_torrent(torrent_id).await?;
    }
    Ok(None)
}
