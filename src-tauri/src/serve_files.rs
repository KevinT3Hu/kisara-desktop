use std::sync::{Arc, Mutex};

use clap::Parser;
use ntex::{
    util::HashMap,
    web::{self, error::ErrorNotFound, types::State, HttpRequest},
};
use ntex_files::NamedFile;
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct ServeState {
    file_maps: Arc<Mutex<HashMap<String, String>>>,
}

impl ServeState {
    fn new() -> Self {
        ServeState {
            file_maps: Default::default(),
        }
    }

    fn files(&self, video: &str, subtitles: &[String]) -> (String, Vec<String>) {
        let mut file_maps = self.file_maps.lock().expect("Failed to lock file maps");
        let mut file_list = Vec::new();
        let mut hasher = Sha256::new();
        hasher.update(video);
        let video_hash = format!("{:x}", hasher.finalize());
        file_maps.insert(video_hash.clone(), video.to_owned());
        for subtitle in subtitles {
            hasher = Sha256::new();
            hasher.update(subtitle);
            let subtitle_hash = format!("{:x}", hasher.finalize());
            file_maps.insert(subtitle_hash.clone(), subtitle.clone());
            file_list.push(subtitle_hash);
        }
        (video_hash, file_list)
    }
}

async fn handle_request(
    state: State<ServeState>,
    req: HttpRequest,
) -> Result<NamedFile, web::Error> {
    let path = req.match_info().query("path");
    let file_maps = state.file_maps.lock().expect("Failed to lock file maps");
    if let Some(file) = file_maps.get(path) {
        Ok(NamedFile::open(file)?)
    } else {
        Err(web::Error::from(ErrorNotFound(format!(
            "File not found: {}",
            path
        ))))
    }
}

#[derive(Parser)]
struct CommandArgs {
    #[clap(short, long)]
    video: String,
    #[clap(short, long)]
    subtitles: Vec<String>,
}

#[ntex::main]
async fn main() {
    println!("Starting server...");
    let args = CommandArgs::parse();
    let serve_state = ServeState::new();
    let files = args
        .subtitles
        .iter()
        .chain(std::iter::once(&args.video))
        .cloned()
        .collect::<Vec<_>>();
    let (video_hash, file_list) = serve_state.files(&args.video, &files);

    let server = web::HttpServer::new(move || {
        web::App::new()
            .state(serve_state.clone())
            .route("/{path}", web::get().to(handle_request))
    });

    eprintln!("{}", video_hash);
    eprintln!("{}", file_list.join(" "));

    println!("Serving files...");
    #[allow(clippy::unwrap_used)]
    server.bind("127.0.0.1:8080").unwrap().run().await.unwrap();
}
