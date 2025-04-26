use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

use crate::error::KisaraResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadConfig {
    pub download_path: String,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        let download_path = {
            #[cfg(debug_assertions)]
            let path = "../downloads";
            #[cfg(not(debug_assertions))]
            let path = "./downloads";

            path
        };
        let download_path =
            std::fs::canonicalize(download_path).expect("Failed to get canonical path");
        if !download_path.exists() {
            std::fs::create_dir_all(&download_path).expect("Failed to create download path");
        }
        Self {
            download_path: download_path.to_string_lossy().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NetworkConfig {
    pub bgm_proxy: Option<String>,
    pub torrents_proxy: Option<String>,
    pub bgm_proxy_enabled: bool,
    pub torrents_proxy_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevelFilter {
    #[default]
    Info,
    Debug,
    Trace,
    Warn,
    Error,
}

impl From<LogLevelFilter> for LevelFilter {
    fn from(level: LogLevelFilter) -> Self {
        match level {
            LogLevelFilter::Info => LevelFilter::INFO,
            LogLevelFilter::Debug => LevelFilter::DEBUG,
            LogLevelFilter::Trace => LevelFilter::TRACE,
            LogLevelFilter::Warn => LevelFilter::WARN,
            LogLevelFilter::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DebugConfig {
    pub log_level: LogLevelFilter,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KisaraConfig {
    pub download_config: DownloadConfig,
    pub network_config: NetworkConfig,
    pub locale: String,
    pub debug_config: DebugConfig,
}

impl Default for KisaraConfig {
    fn default() -> Self {
        Self {
            download_config: DownloadConfig::default(),
            network_config: NetworkConfig::default(),
            locale: sys_locale::get_locale().unwrap_or("zh".to_owned()),
            debug_config: DebugConfig::default(),
        }
    }
}

pub fn load_config() -> KisaraResult<KisaraConfig> {
    let config_path = {
        #[cfg(debug_assertions)]
        let path = "../config.json";
        #[cfg(not(debug_assertions))]
        let path = "./config.json";

        path
    };
    // if config file does not exist, create it with default values
    if !Path::new(config_path).exists() {
        let default_config = KisaraConfig::default();
        let config_str = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(config_path, config_str)?;
        return Ok(default_config);
    }
    // read config file
    let config_str = std::fs::read_to_string(config_path)?;
    // parse config file
    let config: Result<KisaraConfig, _> = serde_json::from_str(&config_str);
    if let Ok(config) = config {
        Ok(config)
    } else {
        // if parse failed, create a new config file with default values
        let default_config = KisaraConfig::default();
        let config_str = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(config_path, config_str)?;
        Ok(default_config)
    }
}

impl KisaraConfig {
    pub fn write_config(&self) -> KisaraResult<()> {
        let config_path = {
            #[cfg(debug_assertions)]
            let path = "../config.json";
            #[cfg(not(debug_assertions))]
            let path = "./config.json";

            path
        };
        // write config file
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_str)?;
        Ok(())
    }
}
