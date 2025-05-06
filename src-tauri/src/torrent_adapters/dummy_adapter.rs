use async_trait::async_trait;

use crate::error::KisaraResult;

use super::{TorrentAdapter, TorrentAdapterFactory, TorrentInfo};

pub struct DummyAdapter;

#[async_trait]
impl TorrentAdapter for DummyAdapter {
    async fn search(&self, _page: u32) -> KisaraResult<Vec<TorrentInfo>> {
        Ok(vec![])
    }
}

pub struct DummyAdapterFactory;

impl DummyAdapterFactory {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<'a> TorrentAdapterFactory<'a> for DummyAdapterFactory {
    fn source_name(&self) -> String {
        "Dummy".to_owned()
    }

    fn create_adapter(
        &self,
        _ep: &'a crate::data::episode::Episode,
        _anime: &'a crate::data::anime::Anime,
        _client: &'a reqwest::Client,
    ) -> Box<dyn TorrentAdapter + Send + Sync + 'a> {
        Box::new(DummyAdapter)
    }
}
