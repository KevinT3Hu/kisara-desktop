use std::str::FromStr;

use chrono::NaiveDate;
use serde::Serialize;

use super::search::AnimeSearchResultItem;

#[derive(Serialize, Debug, Clone)]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub aliases: Vec<String>,
    pub name_cn: String,
    pub image: String,
    pub release_date: Option<NaiveDate>,
}

impl From<AnimeSearchResultItem> for Anime {
    fn from(item: AnimeSearchResultItem) -> Self {
        let aliases = item
            .infobox
            .as_object()
            .and_then(|obj| obj.get("别名"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_object())
                    .filter_map(|v| v.get("v"))
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
        Self {
            id: item.id,
            name: item.name,
            aliases,
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
            aliases: row
                .get::<_, String>(offset + 2)
                .map(|s| serde_json::from_str(&s).unwrap_or_default())?,
            name_cn: row.get(offset + 3)?,
            image: row.get(offset + 4)?,
            release_date: row.get(offset + 5)?,
        })
    }
}
