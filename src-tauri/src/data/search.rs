use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SortType {
    Match,
    Heat,
    Rank,
    Score,
}

impl Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SortType::Match => write!(f, "match"),
            SortType::Heat => write!(f, "heat"),
            SortType::Rank => write!(f, "rank"),
            SortType::Score => write!(f, "score"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Paginated<T> {
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeSearchResultItemImages {
    pub large: String,
    pub common: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeTag {
    pub name: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeRating {
    pub rank: i32,
    pub total: i32,
    pub count: HashMap<String, i32>,
    pub score: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeSearchResultItem {
    pub id: i32,
    pub name: String,
    pub name_cn: String,
    pub summary: String,
    pub date: Option<String>,
    pub images: AnimeSearchResultItemImages,
    pub meta_tags: Vec<String>,
    pub tags: Vec<AnimeTag>,
    pub rating: AnimeRating,
}

#[derive(Serialize, Deserialize)]
pub struct EpisodeSearchResultItem {
    pub id: i32,
    pub name: String,
    pub name_cn: String,
    pub sort: i32,
    pub ep: Option<i32>,
    #[serde(rename = "airdate")]
    pub air_date: String,
}
