use tokio::process::Command;

use crate::error::{KisaraError, KisaraResult};

pub async fn get_video_duration(path: &str) -> KisaraResult<f64> {
    let ffprobe = std::env::current_exe()?
        .parent()
        .expect("Failed to get parent directory")
        .join("ffprobe");
    // ffprobe file_path
    let output = Command::new(ffprobe)
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(KisaraError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let duration = String::from_utf8_lossy(&output.stdout).to_string();
    let duration = duration
        .trim()
        .parse::<f64>()
        .map_err(|_| KisaraError::CommandFailed("Failed to parse duration".to_owned()))?;
    Ok(duration)
}
