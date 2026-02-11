use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let block = Block::default().title("yt-tui")
        .borders(Borders::ALL);

    // Based on the state prints a diffrent mesage
    let text = if app.should_quit {
        "Quitting..."
    } else {
        "Press 'q' to exit"
    };

    let paragraph = Paragraph::new(text)
        .block(block);

    frame.render_widget(paragraph, size);
}
