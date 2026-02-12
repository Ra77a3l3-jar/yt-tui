mod app;
mod ui;
mod yt_dlp;
mod message;

use std::io;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::time::{Duration, Instant};

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    loop {
        while let Ok(msg) = app.reciver.try_recv() {
            match msg {
                message::Message::VideoInfoLoader(info) => {
                    app.video_info = Some(info);
                    app.screen = app::Screen::Normal;
                }
                message::Message::Error(e) => {
                    app.screen = app::Screen::Normal;
                    eprintln!("Error {e}");
                }
            }
        }

        terminal.draw(|frame| ui::render(frame, &app))?;

        if app.should_quit {
            break;
        }

        // Check for q char to change state
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,

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

                            let url = app.input.clone();
                            let sender = app.sender.clone();

                            tokio::spawn(async move {
                                match crate::yt_dlp::info::fetch_info(&url).await {
                                    Ok(info) => {
                                        let _ = sender.send(crate::message::Message::VideoInfoLoader(info)).await;
                                    }
                                    Err(e) => {
                                        let _ = sender.send(crate::message::Message::Error(e.to_string())).await;
                                    }
                                }
                            });
                        }
                    }

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
