use bgm_api::BgmApiClient;
use config::KisaraConfig;
use db::DatabaseHelper;
use tauri::async_runtime::Mutex;

use crate::torrent_adapters::TorrentAdapterRegistry;

pub mod bgm_api;
pub mod config;
pub mod db;
pub mod qbit;

pub type DatabaseHelperState = Mutex<DatabaseHelper>;
pub type BgmApiClientState = Mutex<BgmApiClient>;
pub type TorrentAdapterRegistryState = Mutex<TorrentAdapterRegistry>;
pub type ConfigState = Mutex<KisaraConfig>;
pub type QbitClientState = Mutex<qbit::QbitClient>;
