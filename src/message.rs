use crate::yt_dlp::info::VideoInfo;

pub enum Message {
    VideoInfoLoader(VideoInfo),
    Error(String),
}
