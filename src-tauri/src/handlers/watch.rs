use serde::Serialize;
use tauri::{AppHandle, State};

use crate::{
    data::{anime::Anime, episode::Episode},
    error::KisaraResult,
    states::{ConfigState, DatabaseHelperState, QbitClientState},
    utils::subtitle::transform_subtitles,
};

#[derive(Serialize, Clone)]
pub struct PlayServeInfo {
    pub video: String,
    pub subtitles: Vec<String>,
}

#[derive(Serialize)]
pub struct PlayInfo {
    pub video: String,
    pub subtitles: Vec<String>,
    pub ep: Episode,
    pub anime: Anime,
}

#[tauri::command]
pub async fn parse_torrent_play_info_v2(
    torrent_id: String,
    qbit_client: State<'_, QbitClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    config: State<'_, ConfigState>,
    app: AppHandle,
) -> KisaraResult<PlayInfo> {
    let qbit_client = qbit_client.lock().await;
    let config = config.lock().await;
    let base_dir = &config.download_config.download_path;
    let (video, subtitles) = qbit_client.get_files(&torrent_id, &app).await?;
    let subtitles = transform_subtitles(base_dir, &video, &subtitles).await?;

    let db_helper = db_helper.lock().await;
    let episode = db_helper.get_ep_with_torrent_id(torrent_id.clone()).await?;
    let anime = db_helper.get_anime_with_ep_id(episode.id).await?;

    let play_info = PlayInfo {
        video,
        subtitles,
        ep: episode,
        anime,
    };

    Ok(play_info)
}

#[tauri::command]
pub async fn set_progress(
    ep_id: i32,
    progress: u32,
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<()> {
    let db_helper = db_helper.lock().await;
    db_helper.update_progress(ep_id, progress).await?;
    Ok(())
}
