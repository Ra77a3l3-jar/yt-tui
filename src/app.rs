use crate::yt_dlp::info::VideoInfo;
use tokio::sync::mpsc;
use crate::message::Message;

pub struct App {
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
    pub screen: Screen,
    pub video_info: Option<VideoInfo>,
    pub sender: mpsc::Sender<Message>,
    pub reciver: mpsc::Receiver<Message>,
}

pub enum InputMode {
    Editing,
    Normal,
}

pub enum Screen {
    UrlInput,
    Downloading,
    Normal,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);

        Self {
            should_quit: false,
            input: String::new(),
            input_mode: InputMode::Editing,
            screen: Screen::UrlInput,
            video_info: None,
            sender: tx,
            reciver: rx,
        }
    }
}
