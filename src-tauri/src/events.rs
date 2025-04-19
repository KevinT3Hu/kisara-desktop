use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::error::KisaraResult;

pub trait Event {
    fn emit(self, handle: &AppHandle) -> KisaraResult<()>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TorrentComplete {
    pub id: String,
}

impl TorrentComplete {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Event for TorrentComplete {
    fn emit(self, handle: &AppHandle) -> KisaraResult<()> {
        handle.emit("torrent-complete", self)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TorrentInit {
    pub id: String,
}

impl TorrentInit {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Event for TorrentInit {
    fn emit(self, handle: &AppHandle) -> KisaraResult<()> {
        handle.emit("torrent-init", self)?;
        Ok(())
    }
}
