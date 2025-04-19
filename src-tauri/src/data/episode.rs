use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use super::search::EpisodeSearchResultItem;

#[derive(Serialize)]
pub struct Episode {
    pub id: i32,
    pub anime_id: i32,
    pub sort: i32,
    pub ep: Option<i32>,
    pub name: String,
    pub name_cn: String,
    pub air_date: Option<NaiveDate>,
    pub progress: i32,
    pub last_watch_time: Option<NaiveDateTime>,
    pub torrent_id: Option<String>,
}

impl Episode {
    pub fn from_search_result(item: EpisodeSearchResultItem, anime_id: i32) -> Self {
        Self {
            id: item.id,
            anime_id,
            sort: item.sort,
            ep: item.ep,
            name: item.name,
            name_cn: item.name_cn,
            air_date: NaiveDate::from_str(&item.air_date).ok(),
            progress: 0,
            last_watch_time: None,
            torrent_id: None,
        }
    }

    pub fn from_row(row: &rusqlite::Row, offset: usize) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(offset)?,
            anime_id: row.get(1 + offset)?,
            sort: row.get(2 + offset)?,
            ep: row.get(3 + offset)?,
            name: row.get(4 + offset)?,
            name_cn: row.get(5 + offset)?,
            air_date: row.get(6 + offset)?,
            progress: row.get(7 + offset)?,
            last_watch_time: row.get(8 + offset)?,
            torrent_id: row.get(9 + offset)?,
        })
    }
}
