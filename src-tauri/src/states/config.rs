use std::path::Path;

use langtag::LangTagBuf;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

use crate::error::KisaraResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadConfig {
    pub download_path: String,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        let download_path = Self::default_download_path();
        Self::ensure_download_path_exists(download_path);
        let canonical_path = Self::canonicalize_path(download_path);
        Self {
            download_path: Self::strip_unc_prefix(&canonical_path),
        }
    }
}

impl DownloadConfig {
    const fn default_download_path() -> &'static str {
        #[cfg(debug_assertions)]
        {
            "../downloads"
        }
        #[cfg(not(debug_assertions))]
        {
            "./downloads"
        }
    }

    fn ensure_download_path_exists(path: &str) {
        if !std::fs::exists(path).unwrap_or_default() {
            std::fs::create_dir_all(path).expect("Failed to create download path");
        }
    }

    fn canonicalize_path(path: &str) -> String {
        std::fs::canonicalize(path)
            .expect("Failed to get canonical path")
            .to_string_lossy()
            .to_string()
    }

    fn strip_unc_prefix(path: &str) -> String {
        path.strip_prefix(r"\\?\")
            .map_or_else(|| path.to_owned(), ToString::to_string)
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
            LogLevelFilter::Info => Self::INFO,
            LogLevelFilter::Debug => Self::DEBUG,
            LogLevelFilter::Trace => Self::TRACE,
            LogLevelFilter::Warn => Self::WARN,
            LogLevelFilter::Error => Self::ERROR,
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

fn system_locale() -> String {
    sys_locale::get_locale()
        .map(LangTagBuf::new)
        .and_then(Result::ok)
        .and_then(|t| t.language().map(|l| l.primary().to_lowercase()))
        .unwrap_or_else(|| "zh".to_owned())
}

impl Default for KisaraConfig {
    fn default() -> Self {
        Self {
            download_config: DownloadConfig::default(),
            network_config: NetworkConfig::default(),
            locale: system_locale(),
            debug_config: DebugConfig::default(),
        }
    }
}

pub fn load_config() -> KisaraResult<KisaraConfig> {
    let config_path = KisaraConfig::config_path();
    if !Path::new(config_path).exists() {
        return KisaraConfig::create_default_config(config_path);
    }
    KisaraConfig::read_and_parse_config(config_path)
}

impl KisaraConfig {
    const fn config_path() -> &'static str {
        #[cfg(debug_assertions)]
        {
            "../config.json"
        }
        #[cfg(not(debug_assertions))]
        {
            "./config.json"
        }
    }

    fn create_default_config(config_path: &str) -> KisaraResult<Self> {
        let default_config = Self::default();
        let config_str = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(config_path, config_str)?;
        Ok(default_config)
    }

    fn read_and_parse_config(config_path: &str) -> KisaraResult<Self> {
        let config_str = std::fs::read_to_string(config_path)?;
        serde_json::from_str(&config_str)
            .map_or_else(|_| Self::create_default_config(config_path), Ok)
    }

    pub fn write_config(&self) -> KisaraResult<()> {
        let config_path = Self::config_path();
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_str)?;
        Ok(())
    }
}
