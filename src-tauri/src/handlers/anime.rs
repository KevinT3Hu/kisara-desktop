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
    let (start, end) =
        get_season_start_end(determine_current_season()).expect("Guaranteed to be valid");
    let animes = db_helper
        .lock()
        .await
        .get_animes_between_dates(start, end)
        .await?;

    Ok(animes)
}

#[tauri::command]
pub async fn list_animes(db_helper: State<'_, DatabaseHelperState>) -> KisaraResult<Vec<Anime>> {
    let animes = db_helper.lock().await.list_animes().await?;
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
    let mut suggestions: Vec<String> = client
        .lock()
        .await
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
    let episodes = client
        .lock()
        .await
        .get_episodes(anime.id)
        .await?
        .into_iter()
        .map(|ep| Episode::from_search_result(ep, anime.id))
        .collect::<Vec<_>>();

    db_helper
        .lock()
        .await
        .add_anime_and_episodes(anime, episodes)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn get_episodes(
    client: State<'_, BgmApiClientState>,
    db_helper: State<'_, DatabaseHelperState>,
    anime_id: i32,
) -> KisaraResult<Vec<Episode>> {
    let episodes = client.lock().await.get_episodes(anime_id).await?;

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
    drop(db_helper);
    Ok(eps)
}

#[tauri::command]
pub async fn get_anime(
    client: State<'_, BgmApiClientState>,
    anime_id: i32,
) -> KisaraResult<AnimeSearchResultItem> {
    let anime = client.lock().await.get_anime_info(anime_id).await?;
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
    drop(db_helper);
    Ok(animes)
}

#[tauri::command]
pub async fn get_last_watched_ep(
    db_helper: State<'_, DatabaseHelperState>,
    anime_id: i32,
) -> KisaraResult<Option<i32>> {
    let ep = db_helper.lock().await.get_last_watched_ep(anime_id).await?;
    Ok(ep)
}

#[derive(Serialize)]
pub struct DashboardSummary {
    pub today: Vec<Anime>,
    pub last_watched: Vec<(Anime, Episode)>,
    pub watch_next: Vec<(Anime, Episode)>,
}

#[tauri::command]
pub async fn get_dashboard_summary(
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<DashboardSummary> {
    let db_helper = db_helper.lock().await;
    let today = db_helper.get_today_animes().await?;
    let last_watched = db_helper.get_last_watched().await?;
    let watch_next = db_helper.get_watch_next().await?;
    drop(db_helper);

    Ok(DashboardSummary {
        today,
        last_watched,
        watch_next,
    })
}

#[tauri::command]
pub async fn get_air_calendar(
    db_helper: State<'_, DatabaseHelperState>,
) -> KisaraResult<Vec<Vec<(Anime, Episode)>>> {
    let result = db_helper.lock().await.get_air_calendar().await?;

    Ok(result)
}
