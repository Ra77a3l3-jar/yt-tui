use std::process::Stdio;
use tokio::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoInfo {
    pub title: String,
    pub duration: Option<u64>,
    pub upload_date: Option<String>,
}

pub async fn fetch_info(url: &str) -> Result<VideoInfo, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("yt-dlp")
            .arg("--dump-json")
            .arg(url)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

    if !output.status.success() {
        return Err("yt-dlp failed".into());
    }

    let text = String::from_utf8(output.stdout)?;
    let info: VideoInfo = serde_json::from_str(&text)?;

    Ok(info)
}
