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
    let (anime, ep) = db_helper
        .lock()
        .await
        .get_anime_and_ep_with_ep_id(ep_id)
        .await?;
    let results = registry.lock().await.init_search(&ep, &anime).await?;
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
    let torrent_id = qbit_client.lock().await.add_torrent(&magnet).await?;
    db_helper
        .lock()
        .await
        .set_episode_torrent_id(ep_id, torrent_id)
        .await?;
    Ok(())
}

#[derive(Serialize)]
pub struct TorrentStat {
    pub anime_name: String,
    pub ep: i32,
    pub info: ManagedTorrentInfo,
    pub torrent_id: String,
}

#[tauri::command]
pub async fn get_torrent_stats(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<TorrentStat>> {
    let stats = qbit_client.lock().await.get_torrent_stats();
    let mut stats = stats.into_iter().collect::<Vec<_>>();
    stats.sort_by(|a, b| a.0.cmp(&b.0));

    let db_helper = db_helper.lock().await;
    let mut torrent_stats = Vec::new();
    for (id, torrent) in stats {
        let ep = db_helper.get_ep_with_torrent_id(id.to_string()).await.ok();
        if let Some(ep) = ep {
            let anime = db_helper.get_anime_with_ep_id(ep.id).await?;
            torrent_stats.push(TorrentStat {
                anime_name: anime.name_cn.clone(),
                ep: ep.ep.unwrap_or(ep.sort),
                info: ManagedTorrentInfo {
                    name: torrent.name.clone(),
                    stats: torrent.stats,
                },
                torrent_id: id.to_string(),
            });
        }
    }
    drop(db_helper);
    Ok(torrent_stats)
}

#[tauri::command]
pub async fn remove_torrent(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    torrent_id: String,
) -> KisaraResult<()> {
    qbit_client.lock().await.remove_torrent(&torrent_id).await?;
    db_helper.lock().await.remove_torrent(torrent_id).await?;
    Ok(())
}

#[tracing::instrument(level="info", fields(id=ep_id), skip(qbit_client, db_helper))]
#[tauri::command]
pub async fn torrent_is_present(
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    ep_id: i32,
) -> KisaraResult<Option<String>> {
    let db_helper = db_helper.lock().await;
    let torrent_id = db_helper.get_torrent_id_with_ep_id(ep_id).await?;
    if let Some(torrent_id) = torrent_id {
        if qbit_client.lock().await.torrent_exists(&torrent_id)? {
            return Ok(Some(torrent_id));
        }
        db_helper.remove_torrent(torrent_id).await?;
    }
    drop(db_helper);
    Ok(None)
}
