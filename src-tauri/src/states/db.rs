use std::{collections::BTreeMap, fmt::Debug};

use chrono::NaiveDate;
use itertools::Itertools;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tauri::async_runtime::spawn_blocking;
use tracing::{info, instrument};

use crate::{
    data::{anime::Anime, episode::Episode},
    error::{KisaraError, KisaraResult},
    utils::season::Season,
};

pub struct DatabaseHelper {
    conn_pool: Pool<SqliteConnectionManager>,
}

#[cfg(debug_assertions)]
const DB_PATH: &str = "../db.sqlite"; // to exclude this from being watched by cargo
#[cfg(not(debug_assertions))]
const DB_PATH: &str = "db.sqlite"; // to include this in the release build

impl DatabaseHelper {
    pub fn try_new() -> KisaraResult<Self> {
        let manager = SqliteConnectionManager::file(DB_PATH);
        let pool = Pool::builder().max_size(5).build(manager)?;
        let db_helper = Self { conn_pool: pool };
        db_helper.init_tables()?;
        Ok(db_helper)
    }

    fn init_tables(&self) -> KisaraResult<()> {
        let conn = self.conn_pool.get()?;
        let create_table_stmt = include_str!("sql/create_table.sql");
        conn.execute_batch(create_table_stmt)?;
        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_animes_between_dates(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> KisaraResult<Vec<Anime>> {
        info!("Fetching animes between dates");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM anime WHERE release_date BETWEEN ?1 AND ?2";
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;

            let result = stmt
                .query_map(params![start, end], |row| Anime::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?animes, "Fetched animes between dates");
        Ok(animes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_animes_are_in_list(&self, anime_ids: Vec<i32>) -> KisaraResult<Vec<bool>> {
        info!("Checking if animes are in list");
        let query = "SELECT EXISTS(SELECT 1 FROM anime WHERE id = ?1)";
        let mut result = Vec::new();
        for id in anime_ids {
            let conn = self.conn_pool.get()?;
            let exists = spawn_blocking(move || {
                let mut stmt = conn.prepare(query)?;
                let exists: bool = stmt.query_row(params![id], |row| row.get(0))?;
                KisaraResult::Ok(exists)
            })
            .await??;
            result.push(exists);
        }
        info!(?result, "Checked anime existence in list");
        Ok(result)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn add_anime_and_episodes<T: Into<Anime> + Debug>(
        &self,
        anime: T,
        episodes: Vec<Episode>,
    ) -> KisaraResult<()> {
        info!("Adding anime and episodes");
        let anime = anime.into();
        let conn_pool = self.conn_pool.clone();
        let result: KisaraResult<()> = spawn_blocking(move || {
            let mut conn = conn_pool.get()?;
            let transaction = conn.transaction()?;
            transaction.execute(
                "INSERT INTO anime (id, name, aliases, name_cn, release_date, image) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![anime.id, anime.name, serde_json::to_string(&anime.aliases)?, anime.name_cn, anime.release_date, anime.image],
            )?;
            for episode in episodes {
                transaction.execute(
                    "INSERT INTO episode (id, name, name_cn, sort, air_date, anime_id, ep) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![episode.id, episode.name, episode.name_cn, episode.sort, episode.air_date, episode.anime_id, episode.ep],
                )?;
            }
            transaction.commit()?;
            Ok(())
        })
        .await?;
        info!("Added anime and episodes successfully");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub async fn update_episodes(&self, episodes: Vec<Episode>) -> KisaraResult<()> {
        info!("Updating episodes");
        let conn_pool = self.conn_pool.clone();
        let result: KisaraResult<()> = spawn_blocking(move || {
            let mut conn = conn_pool.get()?;
            let transaction = conn.transaction()?;
            for episode in episodes {
                transaction.execute(
                    "UPDATE episode SET name = ?1, name_cn = ?2, sort = ?3, air_date = ?4 WHERE id = ?5",
                    params![episode.name, episode.name_cn, episode.sort, episode.air_date, episode.id],
                )?;
            }
            transaction.commit()?;
            Ok(())
        })
        .await?;
        info!("Updated episodes successfully");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_episodes(&self, anime_id: i32) -> KisaraResult<Vec<Episode>> {
        info!("Fetching episodes for anime");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM episode WHERE anime_id = ?1";
        let episodes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![anime_id], |row| Episode::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?episodes, "Fetched episodes for anime");
        Ok(episodes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_anime_and_ep_with_ep_id(&self, ep_id: i32) -> KisaraResult<(Anime, Episode)> {
        info!("Fetching anime and episode with episode ID");
        let conn = self.conn_pool.get()?;
        let query =
            "SELECT a.*, e.* FROM anime a JOIN episode e ON a.id = e.anime_id WHERE e.id = ?1";
        let (anime, episode) = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let mut rows = stmt.query(params![ep_id])?;
            if let Some(row) = rows.next()? {
                let anime = Anime::from_row(row, 0)?;
                let episode = Episode::from_row(row, 6)?;
                KisaraResult::Ok((anime, episode))
            } else {
                KisaraResult::Err(KisaraError::NoSuchEpisode(ep_id))
            }
        })
        .await??;
        info!(
            ?anime,
            ?episode,
            "Fetched anime and episode with episode ID"
        );
        Ok((anime, episode))
    }

    #[instrument(level = "info", skip(self))]
    pub async fn set_episode_torrent_id(&self, ep_id: i32, torrent_id: String) -> KisaraResult<()> {
        info!("Setting torrent ID for episode");
        let conn = self.conn_pool.get()?;
        let query = "UPDATE episode SET torrent_id = ?1 WHERE id = ?2";
        spawn_blocking(move || {
            conn.execute(query, params![torrent_id, ep_id])?;
            KisaraResult::Ok(())
        })
        .await??;
        info!("Set torrent ID for episode successfully");
        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_ep_with_torrent_id(&self, torrent_id: String) -> KisaraResult<Episode> {
        info!("Fetching episode with torrent ID");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM episode WHERE torrent_id = ?1";
        let episode = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![torrent_id], |row| Episode::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            if result.is_empty() {
                KisaraResult::Err(KisaraError::NoSuchTorrent(torrent_id))
            } else {
                KisaraResult::Ok(
                    result
                        .into_iter()
                        .next()
                        .expect("We already checked the result is not empty"),
                )
            }
        })
        .await??;
        info!(?episode, "Fetched episode with torrent ID");
        Ok(episode)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_anime_with_ep_id(&self, ep_id: i32) -> KisaraResult<Anime> {
        info!("Fetching anime with episode ID");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM anime WHERE id = (SELECT anime_id FROM episode WHERE id = ?1)";
        let anime = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![ep_id], |row| Anime::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            if result.is_empty() {
                KisaraResult::Err(KisaraError::NoSuchEpisode(ep_id))
            } else {
                KisaraResult::Ok(
                    result
                        .into_iter()
                        .next()
                        .expect("We already checked the result is not empty"),
                )
            }
        })
        .await??;
        info!(?anime, "Fetched anime with episode ID");
        Ok(anime)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn update_progress(&self, ep_id: i32, progress: u32) -> KisaraResult<()> {
        info!("Updating progress for episode");
        let conn = self.conn_pool.get()?;
        // set progress and update last_watch_time
        let query = "UPDATE episode SET progress = ?1, last_watch_time = ?2 WHERE id = ?3";
        let now = chrono::Local::now().naive_local();
        spawn_blocking(move || {
            conn.execute(query, params![progress, now, ep_id])?;
            KisaraResult::Ok(())
        })
        .await??;
        info!("Updated progress for episode successfully");
        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_history(&self) -> KisaraResult<Vec<Episode>> {
        info!("Fetching watch history");
        // get first 10 episodes ordered by last_watch_time desc
        let conn = self.conn_pool.get()?;
        let query =
            "SELECT * FROM episode WHERE last_watch_time IS NOT NULL ORDER BY last_watch_time DESC";
        let episodes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| Episode::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?episodes, "Fetched watch history");
        Ok(episodes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn remove_torrent(&self, torrent_id: String) -> KisaraResult<()> {
        info!("Removing torrent");
        let conn = self.conn_pool.get()?;
        let query = "UPDATE episode SET torrent_id = NULL WHERE torrent_id = ?1";
        spawn_blocking(move || {
            conn.execute(query, params![torrent_id])?;
            KisaraResult::Ok(())
        })
        .await??;
        info!("Removed torrent successfully");
        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_torrent_id_with_ep_id(&self, ep_id: i32) -> KisaraResult<Option<String>> {
        info!("Fetching torrent ID with episode ID");
        let conn = self.conn_pool.get()?;
        let query = "SELECT torrent_id FROM episode WHERE id = ?1";
        let torrent_id = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result: Option<String> = stmt.query_row(params![ep_id], |row| row.get(0))?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?torrent_id, "Fetched torrent ID with episode ID");
        Ok(torrent_id)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn list_animes(&self) -> KisaraResult<Vec<Anime>> {
        info!("Listing all animes");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM anime";
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| Anime::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?animes, "Listed all animes");
        Ok(animes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn list_animes_by_season(&self) -> KisaraResult<BTreeMap<Season, Vec<Anime>>> {
        info!("Listing all animes by season");
        let animes = self.list_animes().await?;
        let mut seasons = BTreeMap::new();
        for anime in animes {
            let season = Season::determine_season(anime.release_date.unwrap_or_default()); // TODO how to handle none release date?
            seasons.entry(season).or_insert_with(Vec::new).push(anime);
        }
        info!(?seasons, "Listed all animes by season");
        Ok(seasons)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_last_watched_ep(&self, anime_id: i32) -> KisaraResult<Option<i32>> {
        let conn = self.conn_pool.get()?;
        let query = "SELECT id FROM episode WHERE anime_id = ?1 AND last_watch_time IS NOT NULL ORDER BY last_watch_time DESC LIMIT 1";
        let episode = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![anime_id], |row| row.get(0))?
                .collect::<Result<Vec<_>, _>>()?;
            if result.is_empty() {
                info!("no last watched episode");
                KisaraResult::Ok(None)
            } else {
                let result = result.into_iter().next();
                info!(?result, "last watched episode");
                KisaraResult::Ok(result)
            }
        })
        .await??;
        Ok(episode)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_today_animes(&self) -> KisaraResult<Vec<Anime>> {
        info!("Fetching today's animes");
        let conn = self.conn_pool.get()?;
        let query = "SELECT * FROM anime WHERE release_date = date('now')";
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| Anime::from_row(row, 0))?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(?animes, "Fetched today's animes");
        Ok(animes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_last_watched(&self) -> KisaraResult<Vec<(Anime, Episode)>> {
        info!("Fetching last watched animes and episodes");
        let conn = self.conn_pool.get()?;
        let query = include_str!("sql/get_last_watched.sql");
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| {
                    let anime = Anime::from_row(row, 0)?;
                    let episode = Episode::from_row(row, 6)?;
                    Ok((anime, episode))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(animes = ?animes, "last watched animes");
        Ok(animes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_air_calendar(&self) -> KisaraResult<Vec<Vec<(Anime, Episode)>>> {
        info!("Fetching air calendar");
        let conn = self.conn_pool.get()?;
        let query = include_str!("sql/get_coming_week.sql");
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| {
                    let anime = Anime::from_row(row, 0)?;
                    let episode = Episode::from_row(row, 6)?;
                    Ok((anime, episode))
                })?
                .flatten()
                .collect::<Vec<_>>();

            let start_date = chrono::Local::now().date_naive();
            let groups = result
                .into_iter()
                .chunk_by(|x| x.1.air_date.expect("Non null"))
                .into_iter()
                .map(|(date, group)| (date, group.collect::<Vec<_>>()))
                .collect::<Vec<_>>();

            let result = (0..7)
                .map(|i| {
                    let date = start_date + chrono::Duration::days(i);

                    #[allow(clippy::pattern_type_mismatch)]
                    groups
                        .iter()
                        .find(|&&(d, _)| d == date)
                        .map(|(_, group)| group.clone())
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>();

            KisaraResult::Ok(result)
        })
        .await??;
        info!(animes = ?animes, "coming week");
        Ok(animes)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_watch_next(&self) -> KisaraResult<Vec<(Anime, Episode)>> {
        info!("Fetching watch next animes and episodes");
        let conn = self.conn_pool.get()?;
        let query = include_str!("sql/get_watch_next.sql");
        let animes = spawn_blocking(move || {
            let mut stmt = conn.prepare(query)?;
            let result = stmt
                .query_map(params![], |row| {
                    let anime = Anime::from_row(row, 0)?;
                    let episode = Episode::from_row(row, 6)?;
                    Ok((anime, episode))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            KisaraResult::Ok(result)
        })
        .await??;
        info!(animes = ?animes, "watch next animes");
        Ok(animes)
    }
}
