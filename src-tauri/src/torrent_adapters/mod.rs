use std::collections::HashMap;

use async_trait::async_trait;
use dummy_adapter::DummyAdapterFactory;
use nyaa_adapter::NyaaAdapterFactory;
use reqwest::{Client, ClientBuilder, Proxy};
use serde::{Deserialize, Serialize};

mod dummy_adapter;
mod nyaa_adapter;

use crate::{
    data::{anime::Anime, episode::Episode},
    error::{KisaraError, KisaraResult},
};

#[derive(Serialize, Deserialize)]
pub struct TorrentInfo {
    pub name: String,
    pub size: Option<String>,
    pub url: Option<String>,
    pub magnet: String,
    pub date: Option<String>,
    pub seeders: Option<u32>,
    pub leechers: Option<u32>,
    pub uploader: Option<String>,
}

#[async_trait]
pub trait TorrentAdapter {
    async fn search(&self, page: u32) -> KisaraResult<Vec<TorrentInfo>>;
}

pub trait TorrentAdapterFactory<'a> {
    fn source_name(&self) -> String;
    fn priority(&self) -> u32 {
        0
    }
    fn create_adapter(
        &self,
        ep: &'a Episode,
        anime: &'a Anime,
        client: &'a Client,
    ) -> Box<dyn TorrentAdapter + Send + Sync + 'a>;
}

pub struct TorrentAdapterRegistry {
    factories: HashMap<String, Box<dyn for<'a> TorrentAdapterFactory<'a> + Send + Sync>>,
    client: Client,
}

impl TorrentAdapterRegistry {
    pub fn new() -> Self {
        let mut builder = ClientBuilder::new();
        if let Ok(p) = Proxy::all("http://127.0.0.1:7890") {
            builder = builder.proxy(p);
        }
        let client = builder.build().expect("Failed to create reqwest client");
        let s = Self {
            factories: HashMap::new(),
            client,
        };
        s.register_adapter(DummyAdapterFactory::new())
            .register_adapter(NyaaAdapterFactory::new())
    }

    fn register_adapter<F>(mut self, factory: F) -> Self
    where
        F: for<'a> TorrentAdapterFactory<'a> + Send + Sync + 'static,
    {
        let name = factory.source_name();
        self.factories.insert(name.clone(), Box::new(factory));
        self
    }

    pub async fn init_search(
        &self,
        ep: &Episode,
        anime: &Anime,
    ) -> KisaraResult<HashMap<String, Vec<TorrentInfo>>> {
        let mut results = HashMap::new();
        let mut factories_vec = self.factories.values().collect::<Vec<_>>();
        factories_vec.sort_by_key(|a| a.priority());
        for factory in factories_vec {
            let name = factory.source_name();
            let adapter = factory.create_adapter(ep, anime, &self.client);
            let torrents = adapter.search(1).await?;
            if torrents.is_empty() {
                continue;
            }
            results.insert(name, torrents);
        }
        Ok(results)
    }

    #[allow(dead_code)] // TODO maybe will use it one day
    pub async fn search(
        &self,
        source: &str,
        ep: &Episode,
        anime: &Anime,
        page: u32,
    ) -> KisaraResult<Vec<TorrentInfo>> {
        if let Some(factory) = self.factories.get(source) {
            let adapter = factory.create_adapter(ep, anime, &self.client);
            let torrents = adapter.search(page).await?;
            Ok(torrents)
        } else {
            Err(KisaraError::NoSuchTorrentAdapter(source.to_owned()))
        }
    }
}

impl Default for TorrentAdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}
