use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Style, Color},
};

use crate::app::{App, InputMode};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        crate::app::Screen::UrlInput => draw_input(frame, app),
        crate::app::Screen::Normal => draw_normal(frame, app),
        crate::app::Screen::Loading => draw_loading(frame, app),
        crate::app::Screen::FormatSelect => draw_formats(frame, app),
    }
}

fn draw_input(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
        .split(size);

    let title = Paragraph::new("YT-TUI")
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
                .title("Enter the link (Press Enter)")
                .borders(Borders::ALL),
        );

    frame.render_widget(input, chunks[1]);

    if let InputMode::Editing = app.input_mode {
        frame.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.input.len() as u16 + 1,
            chunks[1].y + 1,
        );
    }
}

fn draw_loading(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let text = if app.dowloading {
        format!("Downloading... {:.1}%", app.progress)
    } else if let Some(info) = &app.video_info {
        format!("Title:\n{}\n\nPress q to quit", info.title)
    } else {
        "No info found".to_string()
    };

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().title("Status").borders(Borders::ALL));

    frame.render_widget(paragraph, size);
}

fn draw_normal(frame: &mut Frame, app: &App) {
    let size = frame.size();

        let text = if let Some(info) = &app.video_info {
            format!("Title:\n{}\n\nPress q to quit", info.title)
        } else {
            "No info found".to_string()
        };

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().title("Video Info").borders(Borders::ALL));

        frame.render_widget(paragraph, size);
}

fn draw_formats(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let info = match &app.video_info {
        Some(i) => i,
        None => return,
    };

    let items: Vec<String> = info
        .formats
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let marker = if i == app.selected_format { ">>" } else { "  " };

            format!(
                "{} {} | {} | {}",
                marker,
                f.format_id,
                f.ext.as_deref().unwrap_or("?"),
                f.format_note.as_deref().unwrap_or("")
            )
        })
        .collect();

    let text = items.join("\n");

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Select Format").borders(Borders::ALL));

    frame.render_widget(paragraph, size);
}
