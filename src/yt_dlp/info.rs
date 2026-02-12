use std::process::Stdio;
use tokio::process::Command;

pub async fn fetch_info(url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
    Ok(text)
}
