use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
    sync::Arc,
};

use infer::MatcherType;
use librqbit::{
    api::TorrentIdOrHash, AddTorrent, ManagedTorrent, Session, SessionOptions,
    SessionPersistenceConfig, TorrentStats,
};
use serde::Serialize;
use tauri::{async_runtime::spawn, AppHandle};
use tauri_plugin_notification::NotificationExt;
use tracing::{debug, info, info_span, instrument};

use crate::{
    error::{KisaraError, KisaraResult},
    events::{Event, TorrentComplete, TorrentInit},
    utils::video::get_video_duration,
};

use super::config::DownloadConfig;

pub struct QbitClient {
    session: Arc<Session>,
    app: Option<AppHandle>,
    download_folder: PathBuf,
}

impl QbitClient {
    pub async fn new(download_config: DownloadConfig) -> KisaraResult<Self> {
        let mut session_opts: SessionOptions = Default::default();
        let persistence_opts = SessionPersistenceConfig::Json {
            folder: Some(PathBuf::from(&download_config.download_path).join("session")),
        };
        session_opts.persistence = Some(persistence_opts);
        let download_path = PathBuf::from(&download_config.download_path);
        let download_path = std::fs::canonicalize(download_path.clone())
            .map_err(|_| KisaraError::InvalidPath(download_path))?;
        let session =
            Session::new_with_opts(PathBuf::from(download_config.download_path), session_opts)
                .await?;
        let s = Self {
            session: Arc::clone(&session),
            app: None,
            download_folder: download_path,
        };
        session.with_torrents(|torrents| {
            torrents.for_each(|(_, t)| {
                if !t.stats().finished {
                    s.start_new_wait_complete(t);
                }
            });
        });
        Ok(s)
    }

    pub fn set_app(&mut self, app: AppHandle) {
        self.app = Some(app);
    }

    #[instrument(level = "info", skip(self,torrent),fields(torrent_id = torrent.id()))]
    pub fn start_new_wait_init(&self, torrent: Arc<ManagedTorrent>) {
        info!("Starting waiting for init");
        spawn({
            let span = info_span!("torrent_init", torrent_id = torrent.id());
            let app = self.app.clone();

            async move {
                let _enter = span.enter();
                torrent.wait_until_initialized().await?;

                info!("Torrent initialized");
                if let Some(app) = app {
                    TorrentInit::new(torrent.id().to_string()).emit(&app)?;
                }

                KisaraResult::Ok(())
            }
        });
    }

    #[instrument(level = "info", skip(self,torrent),fields(torrent_id = torrent.id()))]
    pub fn start_new_wait_complete(&self, torrent: &Arc<ManagedTorrent>) {
        info!("Starting waiting for completion");
        spawn({
            let span = info_span!("torrent_complete", torrent_id = torrent.id());
            let torrent = Arc::clone(torrent);
            let app = self.app.clone();

            async move {
                let _enter = span.enter();
                torrent.wait_until_completed().await?;

                info!("Torrent completed");
                if let Some(app) = app {
                    TorrentComplete::new(torrent.id().to_string()).emit(&app)?;
                    let _ = app
                        .notification()
                        .builder()
                        .title("Download Complete")
                        .body(format!(
                            "Torrent {} has completed downloading",
                            torrent.name().unwrap_or_default()
                        ))
                        .show();
                }

                KisaraResult::Ok(())
            }
        });
    }

    #[instrument(level = "info", skip(self))]
    pub async fn add_torrent(&self, magnet: &str) -> KisaraResult<String> {
        info!("Adding torrent");
        let handle = self
            .session
            .add_torrent(AddTorrent::Url(Cow::Borrowed(magnet)), None)
            .await?
            .into_handle()
            .expect("We know this is a valid torrent");

        self.start_new_wait_init(Arc::clone(&handle));
        self.start_new_wait_complete(&handle);

        Ok(handle.id().to_string())
    }

    #[instrument(level = "info", skip(self))]
    pub fn get_downloading_torrents(&self) -> u32 {
        let count = RefCell::new(0);
        self.session.with_torrents(|torrents| {
            #[allow(clippy::cast_possible_truncation)]
            count.replace(torrents.filter(|&(_, t)| !t.stats().finished).count() as u32);
        });
        let count = count.into_inner();
        info!("Downloading torrents: {}", count);
        count
    }

    pub fn get_torrent_stats(&self) -> HashMap<usize, ManagedTorrentInfo> {
        let torrents = RefCell::new(HashMap::new());
        self.session.with_torrents(|t| {
            torrents.replace(
                t.map(|(id, t)| {
                    (
                        id,
                        ManagedTorrentInfo {
                            name: t.name(),
                            stats: t.stats(),
                        },
                    )
                })
                .collect(),
            );
        });
        torrents.into_inner()
    }

    // #[instrument(level = "info", skip(self))]
    pub async fn get_files(&self, torrent_id: &str) -> KisaraResult<(String, Vec<String>)> {
        struct VideoTmp {
            pub path: String,
            pub duration: f64,
        }

        impl PartialEq for VideoTmp {
            fn eq(&self, other: &Self) -> bool {
                self.path == other.path
            }
        }

        impl Eq for VideoTmp {}

        impl Ord for VideoTmp {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.duration
                    .partial_cmp(&other.duration)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        }

        impl PartialOrd for VideoTmp {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        // parse torrent_id to usize
        let torrent_id: usize = torrent_id.parse()?;
        let torrent = self
            .session
            .get(TorrentIdOrHash::Id(torrent_id))
            .ok_or(KisaraError::NoSuchTorrent(torrent_id.to_string()))?;
        let files = &torrent
            .metadata
            .load()
            .clone()
            .ok_or(KisaraError::NoSuchTorrent(torrent_id.to_string()))?
            .file_infos;
        debug!(?files, "Files in torrent");

        let mut videos = BinaryHeap::new();
        let mut subtitles = Vec::new();

        for file in files {
            let file_path = self.download_folder.join(&file.relative_filename);

            let file_path = file_path
                .to_str()
                .ok_or(KisaraError::InvalidPath(file_path.clone()))?;

            let kind = infer::get_from_path(file_path)?;
            debug!(?kind, ?file_path, "Inferred file kind");

            if let Some(kind) = kind {
                if matches!(kind.matcher_type(), MatcherType::Video) {
                    let duration = get_video_duration(file_path).await?;
                    debug!(?duration, ?file_path, "Video duration");

                    let video = VideoTmp {
                        path: file_path.to_owned(),
                        duration,
                    };

                    videos.push(video);
                } else if matches!(kind.extension(), "srt" | "sub" | "ass" | "vtt") {
                    debug!(?file_path, "Subtitle file");
                    let subtitle_path = file_path.to_owned();
                    subtitles.push(subtitle_path);
                }
                info!(?file_path, "Match no video or subtitle");
            }
        }

        let video = videos
            .pop()
            .ok_or(KisaraError::NoVideoFoundInTorrent(torrent_id.to_string()))?;

        let video_path = video.path;
        info!(?video_path, "Video path");
        info!(?subtitles, "Subtitles");

        Ok((video_path, subtitles))
    }

    pub async fn remove_torrent(&self, torrent_id: &str) -> KisaraResult<()> {
        let torrent_id: usize = torrent_id.parse()?;
        self.session
            .delete(TorrentIdOrHash::Id(torrent_id), true)
            .await?;

        Ok(())
    }

    pub fn torrent_exists(&self, torrent_id: &str) -> KisaraResult<bool> {
        let torrent_id: usize = torrent_id.parse()?;
        let exists = self.session.get(TorrentIdOrHash::Id(torrent_id)).is_some();
        Ok(exists)
    }
}

#[derive(Serialize)]
pub struct ManagedTorrentInfo {
    pub name: Option<String>,
    pub stats: TorrentStats,
}
