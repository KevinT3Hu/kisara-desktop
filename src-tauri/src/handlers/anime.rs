use serde::Serialize;
use tauri::State;

use crate::{
    data::{
        anime::Anime,
        episode::Episode,
        search::{AnimeSearchResultItem, Paginated, SortType},
    },
    error::KisaraResult,
    states::{BgmApiClientState, DatabaseHelperState},
    utils::datetime::{determine_current_season, get_season_start_end},
};

#[tauri::command]
pub async fn current_season_animes(
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<Anime>> {
    let db_helper = db_helper.lock().await;

    let (start, end) =
        get_season_start_end(determine_current_season()).expect("Guaranteed to be valid");
    let animes = db_helper.get_animes_between_dates(start, end).await?;

    Ok(animes)
}

#[tauri::command]
pub async fn list_animes(db_helper: State<'_, DatabaseHelperState>) -> KisaraResult<Vec<Anime>> {
    let db_helper = db_helper.lock().await;
    let animes = db_helper.list_animes().await?;
    Ok(animes)
}

#[tauri::command]
pub fn current_season() -> String {
    let season = determine_current_season();
    format!("{}Q{}", season.0, season.1)
}

#[tauri::command]
pub async fn search_animes(
    client: State<'_, BgmApiClientState>,
    keyword: String,
    sort: SortType,
    page: Option<u32>,
    limit: Option<u32>,
) -> KisaraResult<Paginated<AnimeSearchResultItem>> {
    let client = client.lock().await;
    client.search_animes(&keyword, sort, page, limit).await
}

#[tauri::command]
pub async fn search_suggestions(
    client: State<'_, BgmApiClientState>,
    keyword: String,
) -> KisaraResult<Vec<String>> {
    let client = client.lock().await;
    let mut suggestions: Vec<String> = client
        .search_animes(&keyword, SortType::Match, None, Some(5))
        .await?
        .data
        .into_iter()
        .map(|item| item.name)
        .collect();
    suggestions.sort();
    suggestions.dedup();
    Ok(suggestions)
}

#[tauri::command]
pub async fn animes_in_list(
    anime_ids: Vec<i32>,
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<bool>> {
    let db_helper = db_helper.lock().await;
    db_helper.get_animes_are_in_list(anime_ids).await
}

#[tauri::command]
pub async fn add_anime(
    anime: AnimeSearchResultItem,
    db_helper: State<'_, DatabaseHelperState>,
    client: State<'_, BgmApiClientState>,
) -> KisaraResult<()> {
    let client = client.lock().await;

    let episodes = client
        .get_episodes(anime.id)
        .await?
        .into_iter()
        .map(|ep| Episode::from_search_result(ep, anime.id))
        .collect::<Vec<_>>();

    let db_helper = db_helper.lock().await;
    db_helper.add_anime_and_episodes(anime, episodes).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_episodes(
    client: State<'_, BgmApiClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    anime_id: i32,
) -> KisaraResult<Vec<Episode>> {
    let client = client.lock().await;
    let episodes = client.get_episodes(anime_id).await?;

    // first update the episodes in the database
    let db_helper = db_helper.lock().await;
    db_helper
        .update_episodes(
            episodes
                .into_iter()
                .map(|ep| Episode::from_search_result(ep, anime_id))
                .collect(),
        )
        .await?;
    // then return the episodes
    let eps = db_helper.get_episodes(anime_id).await?;
    Ok(eps)
}

#[tauri::command]
pub async fn get_anime(
    client: State<'_, BgmApiClientState>,
    anime_id: i32,
) -> KisaraResult<AnimeSearchResultItem> {
    let client = client.lock().await;
    let anime = client.get_anime_info(anime_id).await?;
    Ok(anime)
}

#[tauri::command]
pub async fn get_history(
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<(Anime, Episode)>> {
    let db_helper = db_helper.lock().await;
    let history = db_helper.get_history().await?;

    let mut animes = Vec::new();
    for episode in history {
        let anime = db_helper.get_anime_with_ep_id(episode.id).await?;
        animes.push((anime, episode));
    }
    Ok(animes)
}

#[tauri::command]
pub async fn get_last_watched_ep(
    db_helper: State<'_, DatabaseHelperState>,
    anime_id: i32,
) -> KisaraResult<Option<i32>> {
    let db_helper = db_helper.lock().await;
    let ep = db_helper.get_last_watched_ep(anime_id).await?;
    Ok(ep)
}

#[derive(Serialize)]
pub struct DashboardSummary {
    pub today: Vec<Anime>,
    pub last_watched: Vec<(Anime, Episode)>,
}

#[tauri::command]
pub async fn get_dashboard_summary(
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<DashboardSummary> {
    let db_helper = db_helper.lock().await;
    let today = db_helper.get_today_animes().await?;
    let last_watched = db_helper.get_last_watched().await?;

    Ok(DashboardSummary {
        today,
        last_watched,
    })
}
