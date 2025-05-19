use std::path::{Path, PathBuf};

use tokio::process::Command;
use tracing::info;

use crate::error::{KisaraError, KisaraResult};

pub async fn transform_subtitles(
    base_dir: &str,
    video: &str,
    subtitles: &[String],
) -> KisaraResult<Vec<String>> {
    // create base_dir/subtitles if it doesn't exist
    let base_dir = Path::new(base_dir);
    let subtitles_dir = base_dir.join("subtitles");
    if !subtitles_dir.exists() {
        std::fs::create_dir_all(&subtitles_dir)?;
    }

    // create subtitles/{video} if it doesn't exist
    let video_name = Path::new(video)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or(KisaraError::Any("Invalid video name".to_owned()))?;

    let video_sub_dir = subtitles_dir.join(video_name);
    if !video_sub_dir.exists() {
        std::fs::create_dir_all(&video_sub_dir)?;
    }

    // if video_sub_dir/.kisara exists, directly return all vtt files in video_sub_dir
    let kisara_file = video_sub_dir.join(".kisara");
    if kisara_file.exists() {
        let mut result = Vec::new();
        for entry in std::fs::read_dir(&video_sub_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("vtt") {
                result.push(path.to_string_lossy().to_string());
            }
        }
        return Ok(result);
    }

    let mut result = transform_embedded(&video_sub_dir, video).await?;
    result.extend(transform_external(&video_sub_dir, subtitles).await?);
    // create .kisara file
    std::fs::write(&kisara_file, b"")?;
    info!("Transformed subtitles: {:?}", result);

    Ok(result)
}

async fn transform_embedded(sub_dir: &Path, video: &str) -> KisaraResult<Vec<String>> {
    // get current_executable/ffprobe path
    let ffprobe = std::env::current_exe()?
        .parent()
        .ok_or(KisaraError::Any(
            "Failed to get current executable path".to_owned(),
        ))?
        .join("ffprobe");
    let mut ffprobe = Command::new(ffprobe);
    ffprobe
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("s")
        .arg("-show_entries")
        .arg("stream=index:stream_tags=language,title")
        .arg("-of")
        .arg("csv=p=0")
        .arg(video);

    if cfg!(target_os = "windows") {
        ffprobe.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let ffprobe = ffprobe
        .output()
        .await
        .map_err(|e| KisaraError::Any(format!("Failed to run ffprobe: {}", e)))?;

    if !ffprobe.status.success() {
        return Err(KisaraError::Any(format!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&ffprobe.stderr)
        )));
    }

    let output = String::from_utf8_lossy(&ffprobe.stdout);
    let mut result = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            continue;
        }
        let index = parts[0];
        let lang = parts[0];
        let title = parts[1];

        // check if the subtitle already exists
        let subtitle_path = sub_dir.join(format!("{}.vtt", title));
        if subtitle_path.exists() {
            result.push(subtitle_path.to_string_lossy().to_string());
            continue;
        }

        // extract the subtitle
        let ffmpeg = std::env::current_exe()?
            .parent()
            .ok_or(KisaraError::Any(
                "Failed to get current executable path".to_owned(),
            ))?
            .join("ffmpeg");
        let mut ffmpeg = Command::new(ffmpeg);
        ffmpeg
            .arg("-i")
            .arg(video)
            .arg("-map")
            .arg(format!("0:{}", index))
            .arg("-c:s")
            .arg("webvtt")
            .arg("-metadata")
            .arg(format!("language={}", lang))
            .arg("-metadata")
            .arg(format!("title={}", title))
            .arg(&subtitle_path);

        if cfg!(target_os = "windows") {
            ffmpeg.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let ffmpeg = ffmpeg
            .output()
            .await
            .map_err(|e| KisaraError::Any(format!("Failed to run ffmpeg: {}", e)))?;

        if !ffmpeg.status.success() {
            return Err(KisaraError::Any(format!(
                "ffmpeg failed: {}",
                String::from_utf8_lossy(&ffmpeg.stderr)
            )));
        }

        result.push(subtitle_path.to_string_lossy().to_string());
    }

    Ok(result)
}

async fn transform_external(sub_dir: &Path, subtitles: &[String]) -> KisaraResult<Vec<String>> {
    let mut result = Vec::new();
    for subtitle in subtitles {
        let subtitle_path = PathBuf::from(subtitle);
        if !subtitle_path.exists() {
            return Err(KisaraError::Any(format!(
                "Subtitle file does not exist: {}",
                subtitle
            )));
        }

        // check if the subtitle already exists
        let output_path = sub_dir
            .join(
                subtitle_path
                    .file_name()
                    .expect("We should have a file name"),
            )
            .with_extension("vtt");
        if output_path.exists() {
            result.push(output_path.to_string_lossy().to_string());
            continue;
        }

        // convert the subtitle to vtt format
        let ffmpeg = std::env::current_exe()?
            .parent()
            .ok_or(KisaraError::Any(
                "Failed to get current executable path".to_owned(),
            ))?
            .join("ffmpeg");
        let mut ffmpeg = Command::new(ffmpeg);
        ffmpeg
            .arg("-i")
            .arg(&subtitle_path)
            .arg("-c:s")
            .arg("webvtt")
            .arg(&output_path);

        if cfg!(target_os = "windows") {
            ffmpeg.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let ffmpeg = ffmpeg
            .output()
            .await
            .map_err(|e| KisaraError::Any(format!("Failed to run ffmpeg: {}", e)))?;

        if !ffmpeg.status.success() {
            return Err(KisaraError::Any(format!(
                "ffmpeg failed: {}",
                String::from_utf8_lossy(&ffmpeg.stderr)
            )));
        }

        result.push(output_path.to_string_lossy().to_string());
    }

    Ok(result)
}
