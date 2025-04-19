use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::KisaraResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadConfig {
    pub download_path: String,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        return Self {
            download_path: "../downloads".to_owned(),
        };
        #[cfg(not(debug_assertions))]
        return Self {
            download_path: "./downloads".to_owned(),
        };
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KisaraConfig {
    pub download_config: DownloadConfig,
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
