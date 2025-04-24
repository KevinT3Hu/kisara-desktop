use axum::Router;
use sha2::{Digest, Sha256};
use tokio::{net::TcpListener, sync::oneshot::Receiver};
use tower_http::services::ServeFile;
fn hash_sha2(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn serve_entrypoint(
    video: String,
    subtitles: &[String],
    stop_sig: Receiver<()>,
) -> (String, Vec<String>) {
    let video_hash = hash_sha2(&video);
    let subtitles_hash = subtitles.iter().map(|s| hash_sha2(s)).collect::<Vec<_>>();

    let video_hash_clone = video_hash.clone();
    let _subtitles_hash_clone = subtitles_hash.clone();
    let h = tokio::spawn(async move {
        let mut router = Router::new();
        router = router.route_service(&format!("/{}", video_hash_clone), ServeFile::new(video));

        let listener = TcpListener::bind("localhost:8000").await?;

        axum::serve(listener, router).await?;

        tokio::io::Result::Ok(())
    });

    tokio::spawn(async move {
        let _ = stop_sig.await;
        h.abort();
    });

    (video_hash, subtitles_hash)
}
