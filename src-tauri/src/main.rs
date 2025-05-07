// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(debug_assertions))]
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::ChronoLocal;
#[cfg(not(debug_assertions))]
use tracing_subscriber::{Layer, layer::SubscriberExt, reload, util::SubscriberInitExt};

fn main() {
    // Retaining the WorkerGuard instance (_guard) is crucial for ensuring that the non-blocking logging writer remains active.
    let (reload_handle, _guard) = {
        #[cfg(debug_assertions)]
        {
            // in debug mode, log to stdout
            tracing_subscriber::fmt()
                .with_ansi(false)
                .with_writer(std::io::stdout)
                .with_file(true)
                .with_line_number(true)
                .with_timer(ChronoLocal::rfc_3339())
                .init();
            (None, Option::<WorkerGuard>::None)
        }

        #[cfg(not(debug_assertions))]
        {
            const LOGFILE_SIZE_LIMIT: u64 = 10 * 1024 * 1024; // 10 MB

            if let Ok(file) = std::fs::File::create("kisara.log") {
                // if file is larger than 10 MB, rename it to kisara.log.bak and delete the original kisara.log.bak if exists
                if file.metadata().is_ok_and(|m| m.len() > LOGFILE_SIZE_LIMIT) {
                    #[allow(clippy::let_underscore_must_use)]
                    let _ = std::fs::rename("kisara.log", "kisara.log.bak");
                }
                // create a new file with the same name
                if let Ok(file) = std::fs::File::create("kisara.log") {
                    let (non_blocking, guard) = tracing_appender::non_blocking(file);
                    let layer = tracing_subscriber::fmt::layer()
                        .with_writer(non_blocking)
                        .with_ansi(false)
                        .with_file(true)
                        .with_line_number(true)
                        .with_timer(ChronoLocal::rfc_3339())
                        .with_filter(LevelFilter::INFO);
                    let (layer, reload_handle) = reload::Layer::new(layer);
                    tracing_subscriber::registry().with(layer).init();
                    (Some(reload_handle), Some(guard))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        }
    };

    let r = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime")
        .block_on(kisara_desktop_lib::run(reload_handle));

    if let Err(e) = r {
        tracing::error!("Error: {}", e);
        std::process::exit(1);
    }
}
