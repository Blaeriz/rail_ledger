use std::io;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
mod api;
mod app;
mod config;
mod models;
mod ui;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::{App, Tab};

#[tokio::main]
async fn main() -> Result<()> {
    // Config + API
    let cfg = config::Config::default();
    let api = api::Api::new(cfg)?;
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let mut app = App::new();

    // Draw first frame
    terminal.draw(|f| ui::render(f, &mut app))?;

    // Event loop
    loop {
        // Poll input with small timeout so we can later add periodic refresh tasks
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => { app.running = false; }
                        KeyCode::Tab => { app.tab_index = (app.tab_index + 1) % Tab::all().len(); }
                        KeyCode::BackTab => {
                            app.tab_index = if app.tab_index == 0 { Tab::all().len() - 1 } else { app.tab_index - 1 };
                        }
                        KeyCode::Char('g') => {
                            // Placeholder for g+key navigation; future implementation
                            app.status = "g- navigation: not yet implemented".into();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    let idx = app.batch_state.selected().unwrap_or(0);
                                    let new_idx = idx.saturating_sub(1);
                                    app.batch_state.select(Some(new_idx));
                                    app.status = format!("Row {}/{}", new_idx + 1, len);
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    let idx = app.batch_state.selected().unwrap_or(0);
                                    let new_idx = (idx + 1).min(len.saturating_sub(1));
                                    app.batch_state.select(Some(new_idx));
                                    app.status = format!("Row {}/{}", new_idx + 1, len);
                                }
                            }
                        }
                        KeyCode::PageUp => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    let idx = app.batch_state.selected().unwrap_or(0);
                                    let new_idx = idx.saturating_sub(10);
                                    app.batch_state.select(Some(new_idx));
                                    app.status = format!("Row {}/{}", new_idx + 1, len);
                                }
                            }
                        }
                        KeyCode::PageDown => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    let idx = app.batch_state.selected().unwrap_or(0);
                                    let new_idx = (idx + 10).min(len.saturating_sub(1));
                                    app.batch_state.select(Some(new_idx));
                                    app.status = format!("Row {}/{}", new_idx + 1, len);
                                }
                            }
                        }
                        KeyCode::Home => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    app.batch_state.select(Some(0));
                                    app.status = format!("Row {}/{}", 1, len);
                                }
                            }
                        }
                        KeyCode::End => {
                            if matches!(app.current_tab(), Tab::Batches) {
                                let len = app.batches.len();
                                if len > 0 {
                                    let last = len - 1;
                                    app.batch_state.select(Some(last));
                                    app.status = format!("Row {}/{}", last + 1, len);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if !app.running { break; }

        // Refresh batches periodically while on Batches tab
        if matches!(app.current_tab(), Tab::Batches) && app.batches.is_empty() {
            match api.batches().await {
                Ok(items) => {
                    app.batches = items;
                    if !app.batches.is_empty() {
                        app.batch_state.select(Some(0));
                    }
                    app.status = format!("Loaded {} batches", app.batches.len());
                }
                Err(e) => {
                    app.status = format!("Error loading batches: {}", e);
                }
            }
        }

        terminal.draw(|f| ui::render(f, &mut app))?;
    }

    // Restore terminal
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
