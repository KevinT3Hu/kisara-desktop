use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use tokio::{spawn, sync::oneshot::Receiver};

use crate::{error::KisaraResult, handlers::PlayServeInfo};

pub fn serve_video(info: &PlayServeInfo, stop_sig: Receiver<()>) -> KisaraResult<PlayServeInfo> {
    let mut command = Command::new("serve_files");
    command.arg("--video").arg(&info.video);
    for subtitle in &info.subtitles {
        command.arg("--subtitles").arg(subtitle);
    }
    command.stderr(Stdio::piped()).stdin(Stdio::null());

    let mut child = command.spawn()?;
    println!("Started serve_files process with PID: {}", child.id());
    // get two lines of output
    let stderr = child.stderr.take().expect("Should have stderr");
    let mut reader = BufReader::new(stderr);
    let mut video_hash = String::new();
    let mut subtitle_hashes = String::new();
    // read the first line
    reader.read_line(&mut video_hash)?;
    // read the second line
    reader.read_line(&mut subtitle_hashes)?;
    let video_hash = video_hash.trim().to_owned();
    println!("Video hash: {}", video_hash);
    let subtitle_hashes = subtitle_hashes
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    spawn(async move {
        // wait for the stop signal
        stop_sig.await.expect("Failed to receive stop signal");
        println!("Received stop signal");
        // kill the child process
        child.kill().expect("Failed to kill child process");
    });

    Ok(PlayServeInfo {
        video: video_hash,
        subtitles: subtitle_hashes,
    })
}
