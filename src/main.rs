mod app;
mod ui;

use std::io;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if app.should_quit {
            break;
        }

        // Check for q char to change state
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                KeyCode::Char(c) => {
                    if matches!(app.input_mode, app::InputMode::Editing) {
                        app.input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if matches!(app.input_mode, app::InputMode::Editing) {
                        app.input.pop();
                    }
                }
                KeyCode::Enter => {
                    if matches!(app.screen, app::Screen::UrlInput) {
                        app.input_mode = app::InputMode::Normal;
                        app.screen = app::Screen::Downloading;
                    }
                }
                _=> {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
