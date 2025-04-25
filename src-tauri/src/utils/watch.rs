use tokio::sync::oneshot::Receiver;

use crate::handlers::PlayServeInfo;

use super::serve_files::serve_entrypoint;

pub fn serve_video(info: &PlayServeInfo, stop_sig: Receiver<()>) -> PlayServeInfo {
    let (video_hash, subtitle_hashes) = serve_entrypoint(&info.video, &info.subtitles, stop_sig);

    PlayServeInfo {
        video: video_hash,
        subtitles: subtitle_hashes,
    }
}
