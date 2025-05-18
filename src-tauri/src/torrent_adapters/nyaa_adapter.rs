use async_trait::async_trait;
use futures::future::join_all;
use kuchikiki::traits::TendrilSink;
use reqwest::Client;
use tracing::{info, instrument};

use crate::{
    data::{anime::Anime, episode::Episode},
    error::{KisaraError, KisaraResult},
};

use super::{TorrentAdapter, TorrentAdapterFactory, TorrentInfo};

pub struct NyaaAdapter<'a> {
    ep: &'a Episode,
    anime: &'a Anime,
    client: &'a Client,
}

impl NyaaAdapter<'_> {
    #[instrument(level = "info", skip(self))]
    async fn search_keyword(&self, keyword: String) -> KisaraResult<Vec<TorrentInfo>> {
        info!("Searching");
        let url = format!("https://nyaa.si/?f=0&c=1_0&q={}&s=seeders&o=desc", keyword);
        let response = self.client.get(&url).send().await?.text().await?;
        Self::parse(&response)
    }

    fn parse(html: &str) -> KisaraResult<Vec<TorrentInfo>> {
        let doc = kuchikiki::parse_html().one(html);

        let table = doc.select_first("table.torrent-list").map_err(|_| {
            KisaraError::HtmlParseError("Failed to select table.torrent-list".to_owned())
        })?;

        let ret = table
            .as_node()
            .select("tr")
            .map_err(|_| {
                KisaraError::HtmlParseError(
                    "Failed to select rows in table.torrent-list".to_owned(),
                )
            })?
            .filter_map(|node| {
                let row = node.as_node();

                // select second td and first <a> child tag of the td whose class is not comments
                let (name, url) = row
                    .select("td:nth-child(2) > a:not(.comments)")
                    .map_err(|_| {
                        KisaraError::HtmlParseError("Failed to select name and url".to_owned())
                    })
                    .ok()
                    .and_then(|mut node| node.next())
                    .map(|node| {
                        (
                            node.text_contents(),
                            node.attributes
                                .borrow()
                                .get("href")
                                .map(|s| format!("{}{}", "https://nyaa.si", s)),
                        )
                    })?;

                // a tag that href starts with magnet
                let magnet = row
                    .select("td:nth-child(3) > a[href^=\"magnet:\"]")
                    .map_err(|_| {
                        KisaraError::HtmlParseError("Failed to select magnet link".to_owned())
                    })
                    .ok()
                    .and_then(|mut node| node.next())
                    .and_then(|node| node.attributes.borrow().get("href").map(ToOwned::to_owned))?;

                let size = row
                    .select("td:nth-child(4)")
                    .map_err(|_| KisaraError::HtmlParseError("Failed to select size".to_owned()))
                    .ok()
                    .and_then(|mut node| node.next())
                    .map(|node| node.text_contents());

                let date = row
                    .select("td:nth-child(5)")
                    .map_err(|_| KisaraError::HtmlParseError("Failed to select date".to_owned()))
                    .ok()
                    .and_then(|mut node| node.next())
                    .map(|node| node.text_contents());

                let seeders = row
                    .select("td:nth-child(6)")
                    .map_err(|_| KisaraError::HtmlParseError("Failed to select seeders".to_owned()))
                    .ok()
                    .and_then(|mut node| node.next())
                    .and_then(|node| node.text_contents().parse::<u32>().ok());

                let leechers = row
                    .select("td:nth-child(7)")
                    .map_err(|_| {
                        KisaraError::HtmlParseError("Failed to select leechers".to_owned())
                    })
                    .ok()
                    .and_then(|mut node| node.next())
                    .and_then(|node| node.text_contents().parse::<u32>().ok());

                Some(TorrentInfo {
                    name,
                    size,
                    url,
                    magnet,
                    date,
                    seeders,
                    leechers,
                    uploader: None,
                })
            })
            .collect::<Vec<_>>();
        info!("Parsed {} torrents", ret.len());

        Ok(ret)
    }
}

#[async_trait]
impl TorrentAdapter for NyaaAdapter<'_> {
    #[instrument(level = "info", skip(self))]
    async fn search(&self, _page: u32) -> KisaraResult<Vec<TorrentInfo>> {
        let ep = self.ep.ep.unwrap_or(self.ep.sort);
        // if ep is 1-digit, add 0 in front of it
        let ep = if ep < 10 {
            format!("0{}", ep)
        } else {
            ep.to_string()
        };
        let keyword = format!("{} \"{}\"", self.anime.name, ep);
        let keyword_cn = format!("{} \"{}\"", self.anime.name_cn, ep);
        let extra_keywords = self
            .anime
            .keywords
            .iter()
            .map(|k| format!("{}, \"{}\"", k, ep));

        let mut final_keywords = self
            .anime
            .aliases
            .iter()
            .map(|alias| format!("{} \"{}\"", alias, ep))
            .chain(vec![keyword, keyword_cn])
            .chain(extra_keywords)
            .collect::<Vec<_>>();

        // remove duplicates
        final_keywords.sort();
        final_keywords.dedup();

        // remove empty strings
        final_keywords.retain(|s| !s.is_empty());

        info!(keywords = ?final_keywords);

        let mut results = join_all(
            final_keywords
                .into_iter()
                .map(|keyword| self.search_keyword(keyword)),
        )
        .await
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect::<Vec<_>>();

        results.sort_unstable_by_key(|a| a.magnet.clone());
        results.dedup_by_key(|a| a.magnet.clone());
        results.sort_unstable_by_key(|a| a.seeders.unwrap_or(0));
        results.reverse();

        info!("Found {} torrents", results.len());

        Ok(results)
    }
}

pub struct NyaaAdapterFactory;

impl NyaaAdapterFactory {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<'a> TorrentAdapterFactory<'a> for NyaaAdapterFactory {
    fn source_name(&self) -> String {
        "Nyaa".to_owned()
    }

    fn create_adapter(
        &self,
        ep: &'a Episode,
        anime: &'a Anime,
        client: &'a Client,
    ) -> Box<dyn TorrentAdapter + Send + Sync + 'a> {
        Box::new(NyaaAdapter { ep, anime, client })
    }
}
