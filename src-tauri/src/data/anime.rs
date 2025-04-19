use std::str::FromStr;

use chrono::NaiveDate;
use serde::Serialize;

use super::search::AnimeSearchResultItem;

#[derive(Serialize)]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub name_cn: String,
    pub image: String,
    pub release_date: Option<NaiveDate>,
}

impl From<AnimeSearchResultItem> for Anime {
    fn from(item: AnimeSearchResultItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            name_cn: item.name_cn,
            image: item.images.common,
            release_date: item.date.and_then(|d| NaiveDate::from_str(&d).ok()), // Placeholder date
        }
    }
}

impl Anime {
    pub fn from_row(row: &rusqlite::Row, offset: usize) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(offset)?,
            name: row.get(offset + 1)?,
            name_cn: row.get(offset + 2)?,
            image: row.get(offset + 3)?,
            release_date: row.get(offset + 4)?,
        })
    }
}
