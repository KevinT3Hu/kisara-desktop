use serde::Serialize;

pub type KisaraResult<T> = Result<T, KisaraError>;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum KisaraError {
    #[error("Database error: {0}")]
    SqliteConnPoolError(#[from] r2d2::Error),

    #[error("Database error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("HTML parse error: {0}")]
    HtmlParseError(String),

    #[error("No such torrent adapter: {0}")]
    NoSuchTorrentAdapter(String),

    #[error("No such episode with ID: {0}")]
    NoSuchEpisode(i32),

    #[error("No such episode with torrent ID: {0}")]
    NoSuchTorrent(String),

    #[error("StdIO error: {0}")]
    StdIOError(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Error: {0}")]
    Fallback(#[from] anyhow::Error),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Invalid Path: {0}")]
    InvalidPath(std::path::PathBuf),

    #[error("Shell Exec Error: {0}")]
    ShellExecError(#[from] tauri_plugin_shell::Error),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("No video found in torrent: {0}")]
    NoVideoFoundInTorrent(String),

    #[error("{0}")]
    Any(String),
}

impl Serialize for KisaraError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
