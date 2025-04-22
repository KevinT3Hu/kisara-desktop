use reqwest::{Client, ClientBuilder};
use serde_json::json;

use crate::{
    data::search::{AnimeSearchResultItem, EpisodeSearchResultItem, Paginated, SortType},
    error::KisaraResult,
};

pub struct BgmApiClient {
    client: Client,
}

impl BgmApiClient {
    pub fn new(proxy: Option<String>) -> Self {
        let mut builder = ClientBuilder::new()
            .user_agent("Kisara/0.1")
            .timeout(std::time::Duration::from_secs(10));

        if let Some(proxy_url) = proxy {
            if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        let client = builder.build().expect("Failed to create reqwest client");
        BgmApiClient { client }
    }

    pub async fn search_animes(
        &self,
        keyword: &str,
        sort: SortType,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> KisaraResult<Paginated<AnimeSearchResultItem>> {
        let limit = limit.unwrap_or(20);
        let offset = page.unwrap_or(1) * limit - limit;

        let url = format!(
            "https://api.bgm.tv/v0/search/subjects?offset={}&limit={}",
            offset, limit
        );

        let result: Paginated<AnimeSearchResultItem> = self
            .client
            .post(&url)
            .json(&json!({
                "keyword": keyword,
                "sort": sort.to_string(),
                "filter": {
                    "type": [2],
                    "nsfw": false,
                }
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    pub async fn get_episodes(&self, anime_id: i32) -> KisaraResult<Vec<EpisodeSearchResultItem>> {
        let url = "https://api.bgm.tv/v0/episodes";
        let result: Paginated<EpisodeSearchResultItem> = self
            .client
            .get(url)
            .query(&[
                ("subject_id", anime_id),
                ("type", 0),
                ("limit", 100),
                ("offset", 0),
            ])
            .send()
            .await?
            .json()
            .await?;

        // get all episodes
        let mut episodes = Vec::new();
        episodes.extend(result.data);
        let mut offset = result.offset + result.limit;
        while offset < result.total {
            let result: Paginated<EpisodeSearchResultItem> = self
                .client
                .get(url)
                .query(&[
                    ("subject_id", anime_id),
                    ("type", 0),
                    ("limit", 100),
                    #[allow(clippy::cast_possible_wrap)]
                    ("offset", offset as i32),
                ])
                .send()
                .await?
                .json()
                .await?;
            episodes.extend(result.data);
            offset += result.limit;
        }
        Ok(episodes)
    }

    pub async fn get_anime_info(&self, anime_id: i32) -> KisaraResult<AnimeSearchResultItem> {
        let url = format!("https://api.bgm.tv/v0/subjects/{}", anime_id);
        let result: AnimeSearchResultItem = self.client.get(&url).send().await?.json().await?;
        Ok(result)
    }
}
