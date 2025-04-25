use axum::Router;
use tokio::{net::TcpListener, sync::oneshot::Receiver};
use tower_http::{cors::CorsLayer, services::ServeFile};
use tracing::info;

const FILE_SEPARATORS: &[char] = &['/', '\\'];

pub fn serve_entrypoint(
    video: &str,
    subtitles: &[String],
    stop_sig: Receiver<()>,
) -> (String, Vec<String>) {
    info!("Serving video: {}, subtitles: {:?}", video, subtitles);
    let subtitles_clone = subtitles.to_owned();
    let subtitles_names = subtitles
        .iter()
        .map(|s| {
            s.split(FILE_SEPARATORS)
                .next_back()
                .unwrap_or("subtitle.vtt")
                .to_owned()
        })
        .map(|s| urlencoding::encode(&s).to_string())
        .collect::<Vec<_>>();
    let video_clone = video.to_owned();
    let video_name = video
        .split(FILE_SEPARATORS)
        .next_back()
        .unwrap_or("video.mp4")
        .to_owned();
    let video_name = urlencoding::encode(&video_name).to_string();

    let video_name_clone = video_name.clone();
    let subtitles_names_clone = subtitles_names.clone();
    let h = tokio::spawn(async move {
        let mut router = Router::new();
        router = router.route_service(
            &format!("/{}", video_name_clone),
            ServeFile::new(video_clone),
        );

        for (index, subtitle) in subtitles_clone.iter().enumerate() {
            let name = subtitles_names_clone[index].clone();
            router = router.route_service(&format!("/{}", name), ServeFile::new(subtitle));
        }

        router = router.layer(CorsLayer::permissive());

        let listener = TcpListener::bind("localhost:8000").await?;

        axum::serve(listener, router).await?;

        tokio::io::Result::Ok(())
    });

    tokio::spawn(async move {
        let _ = stop_sig.await;
        h.abort();
    });

    (video_name, subtitles_names)
}
