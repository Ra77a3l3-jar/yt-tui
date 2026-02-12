use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Style, Color},
};

use crate::app::{App, InputMode};

pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(size);

    let title = Paragraph::new("yt-tui")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, chunks[0]);

    let input_style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::LightRed),
        InputMode::Normal => Style::default().fg(Color::Green),
    };

    let input = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .title("Enter the youtube link (Press Enter)")
                .borders(Borders::ALL),
        );
    frame.render_widget(input, chunks[1]);
}
