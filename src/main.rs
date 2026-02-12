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
use tokio::io::Stdout;
use std::time::{Duration, Instant};
use crate::message::Message;

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
                    app.screen = app::Screen::FormatSelect;
                }
                message::Message::Error(e) => {
                    app.screen = app::Screen::Normal;
                    eprintln!("Error {e}");
                }
                message::Message::Progress(p) => {
                    app.progress = p;
                }
                message::Message::DownloadFinished => {
                    app.dowloading = false;
                    app.screen = app::Screen::Normal;
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

                    KeyCode::Up => {
                        if matches!(app.screen, app::Screen::FormatSelect) {
                            if app.selected_format > 0 {
                                app.selected_format -= 1;
                            }
                        }
                    }

                    KeyCode::Down => {
                        if matches!(app.screen, app::Screen::FormatSelect) {
                            if let Some(info) = &app.video_info {
                                if app.selected_format + 1 < info.formats.len() {
                                    app.selected_format += 1;
                                }
                            }
                        }
                    }

                    KeyCode::Enter => {
                        if matches!(app.screen, app::Screen::FormatSelect) {
                            app.screen = app::Screen::Loading;
                            app.dowloading = true;

                            let sender = app.sender.clone();
                            let video_info = app.video_info.clone().unwrap();
                            let selected = app.selected_format;

                            tokio::spawn(async move {
                                let format_id = &video_info.formats[selected].format_id;
                                let url = video_info.title.clone();

                                // Create yt-dlp sub process
                                let mut cmd = tokio::process::Command::new("yt-dlp");
                                cmd.arg("-f").arg(format_id);
                                cmd.arg(&url);
                                cmd.stdout(std::process::Stdio::piped());
                                cmd.stderr(std::process::Stdio::piped());

                                let mut child = cmd.spawn().expect("Failed to spawn yt-dlp");

                                if let Some(stdout) = child.stdout.take() {
                                    use tokio::io::{BufReader, AsyncBufReadExt};
                                    let reader = BufReader::new(stdout);
                                    let mut lines = reader.lines();

                                    while let Ok(Some(line)) = lines.next_line().await {
                                        // Get progress from line
                                        if let Some(percent) = parse_progress(&line) {
                                            let _ = sender.send(message::Message::Progress(percent)).await;
                                        }
                                    }
                                }
                                let _ = child.wait().await;
                                let _ = sender.send(Message::DownloadFinished).await;
                            });
                        }

                        if matches!(app.screen, app::Screen::UrlInput) {
                            app.input_mode = app::InputMode::Normal;
                            app.screen = app::Screen::Loading;

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

fn parse_progress(line: &str) -> Option<f64> {
    // It will return the value from lines like "download" 50.0%
    if line.contains("%") {
        let parts: Vec<_> = line.split_whitespace().collect();
        for p in parts {
            if let Ok(val) = p.trim_end_matches('%').parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}
