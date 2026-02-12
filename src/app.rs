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
    pub spinner_index: usize,
    pub selected_format: usize,
}

pub enum InputMode {
    Editing,
    Normal,
}

pub enum Screen {
    UrlInput,
    Normal,
    Loading,
    FormatSelect,
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
            spinner_index: 0,
            selected_format: 0,
        }
    }

    pub fn spinner_frame(&self) -> &'static str {
        const FRAMES: [&str; 10] = [
            "⠋","⠙","⠹","⠸","⠼",
            "⠴","⠦","⠧","⠇","⠏",
        ];

        FRAMES[self.spinner_index % FRAMES.len()]
    }

    pub fn tick(&mut self) {
        self.spinner_index += 1;
    }
}
