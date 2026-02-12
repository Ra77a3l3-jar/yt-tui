pub struct App {
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
}

pub enum InputMode {
    Editing,
    Normal,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            input: String::new(),
            input_mode: InputMode::Editing,
        }
    }
}
