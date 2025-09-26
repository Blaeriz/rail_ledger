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

use app::{App, Tab, PAGE_SIZE};

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
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        let idx = app.batch_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(1);
                                        app.batch_state.select(Some(new_idx));
                                        app.batch_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        let idx = app.vendor_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(1);
                                        app.vendor_state.select(Some(new_idx));
                                        app.vendor_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        let idx = app.report_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(1);
                                        app.report_state.select(Some(new_idx));
                                        app.report_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        let idx = app.batch_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 1).min(len.saturating_sub(1));
                                        app.batch_state.select(Some(new_idx));
                                        app.batch_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        let idx = app.vendor_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 1).min(len.saturating_sub(1));
                                        app.vendor_state.select(Some(new_idx));
                                        app.vendor_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        let idx = app.report_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 1).min(len.saturating_sub(1));
                                        app.report_state.select(Some(new_idx));
                                        app.report_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::PageUp => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        let idx = app.batch_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(10);
                                        app.batch_state.select(Some(new_idx));
                                        app.batch_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        let idx = app.vendor_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(10);
                                        app.vendor_state.select(Some(new_idx));
                                        app.vendor_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        let idx = app.report_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(10);
                                        app.report_state.select(Some(new_idx));
                                        app.report_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::PageDown => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        let idx = app.batch_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 10).min(len.saturating_sub(1));
                                        app.batch_state.select(Some(new_idx));
                                        app.batch_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        let idx = app.vendor_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 10).min(len.saturating_sub(1));
                                        app.vendor_state.select(Some(new_idx));
                                        app.vendor_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        let idx = app.report_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 10).min(len.saturating_sub(1));
                                        app.report_state.select(Some(new_idx));
                                        app.report_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        if app.batch_page > 0 {
                                            app.batch_page -= 1;
                                            let new_idx = app.batch_page * PAGE_SIZE;
                                            let new_idx = new_idx.min(len.saturating_sub(1));
                                            app.batch_state.select(Some(new_idx));
                                            let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                            app.status = format!("Page {}/{} · Row {}/{}", app.batch_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        if app.vendor_page > 0 {
                                            app.vendor_page -= 1;
                                            let new_idx = app.vendor_page * PAGE_SIZE;
                                            let new_idx = new_idx.min(len.saturating_sub(1));
                                            app.vendor_state.select(Some(new_idx));
                                            let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                            app.status = format!("Page {}/{} · Row {}/{}", app.vendor_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        if app.report_page > 0 {
                                            app.report_page -= 1;
                                            let new_idx = app.report_page * PAGE_SIZE;
                                            let new_idx = new_idx.min(len.saturating_sub(1));
                                            app.report_state.select(Some(new_idx));
                                            let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                            app.status = format!("Page {}/{} · Row {}/{}", app.report_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 {
                                        let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                        if app.batch_page + 1 < total_pages {
                                            app.batch_page += 1;
                                            let mut new_idx = app.batch_page * PAGE_SIZE;
                                            new_idx = new_idx.min(len.saturating_sub(1));
                                            app.batch_state.select(Some(new_idx));
                                            app.status = format!("Page {}/{} · Row {}/{}", app.batch_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 {
                                        let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                        if app.vendor_page + 1 < total_pages {
                                            app.vendor_page += 1;
                                            let mut new_idx = app.vendor_page * PAGE_SIZE;
                                            new_idx = new_idx.min(len.saturating_sub(1));
                                            app.vendor_state.select(Some(new_idx));
                                            app.status = format!("Page {}/{} · Row {}/{}", app.vendor_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 {
                                        let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                        if app.report_page + 1 < total_pages {
                                            app.report_page += 1;
                                            let mut new_idx = app.report_page * PAGE_SIZE;
                                            new_idx = new_idx.min(len.saturating_sub(1));
                                            app.report_state.select(Some(new_idx));
                                            app.status = format!("Page {}/{} · Row {}/{}", app.report_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Home => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 { app.batch_state.select(Some(0)); app.batch_page = 0; app.status = format!("Row {}/{}", 1, len); }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 { app.vendor_state.select(Some(0)); app.vendor_page = 0; app.status = format!("Row {}/{}", 1, len); }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 { app.report_state.select(Some(0)); app.report_page = 0; app.status = format!("Row {}/{}", 1, len); }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::End => {
                            match app.current_tab() {
                                Tab::Batches => {
                                    let len = app.batches.len();
                                    if len > 0 { let last = len - 1; app.batch_state.select(Some(last)); app.batch_page = last / PAGE_SIZE; app.status = format!("Row {}/{}", last + 1, len); }
                                }
                                Tab::Vendors => {
                                    let len = app.vendors.len();
                                    if len > 0 { let last = len - 1; app.vendor_state.select(Some(last)); app.vendor_page = last / PAGE_SIZE; app.status = format!("Row {}/{}", last + 1, len); }
                                }
                                Tab::Reports => {
                                    let len = app.reports.len();
                                    if len > 0 { let last = len - 1; app.report_state.select(Some(last)); app.report_page = last / PAGE_SIZE; app.status = format!("Row {}/{}", last + 1, len); }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if !app.running { break; }

        // Lazy-load per tab (can be extended to timed refresh later)
        // Lazy-load per tab (can be extended to timed refresh later)
        if matches!(app.current_tab(), Tab::Overview) {
            // For overview, try to have vendors and reports for stats and recent list
            if app.vendors.is_empty() {
                match api.vendors().await {
                    Ok(items) => {
                        app.vendors = items;
                        if !app.vendors.is_empty() { app.vendor_state.select(Some(0)); app.vendor_page = 0; }
                        app.status = format!("Loaded {} vendors", app.vendors.len());
                    }
                    Err(e) => { app.status = format!("Error loading vendors: {}", e); }
                }
            }
            if app.reports.is_empty() {
                match api.reports().await {
                    Ok(items) => {
                        app.reports = items;
                        if !app.reports.is_empty() { app.report_state.select(Some(0)); app.report_page = 0; }
                        app.status = format!("Loaded {} reports", app.reports.len());
                    }
                    Err(e) => { app.status = format!("Error loading reports: {}", e); }
                }
            }
            if app.batches.is_empty() {
                match api.batches().await {
                    Ok(items) => {
                        app.batches = items;
                        if !app.batches.is_empty() { app.batch_state.select(Some(0)); app.batch_page = 0; }
                        app.status = format!("Loaded {} batches", app.batches.len());
                    }
                    Err(e) => { app.status = format!("Error loading batches: {}", e); }
                }
            }
        }
        if matches!(app.current_tab(), Tab::Batches) && app.batches.is_empty() {
            match api.batches().await {
                Ok(items) => {
                    app.batches = items;
                    if !app.batches.is_empty() {
                        app.batch_state.select(Some(0));
                        app.batch_page = 0;
                    }
                    app.status = format!("Loaded {} batches", app.batches.len());
                }
                Err(e) => {
                    app.status = format!("Error loading batches: {}", e);
                }
            }
        }
        if matches!(app.current_tab(), Tab::Vendors) && app.vendors.is_empty() {
            match api.vendors().await {
                Ok(items) => {
                    app.vendors = items;
                    if !app.vendors.is_empty() { app.vendor_state.select(Some(0)); app.vendor_page = 0; }
                    app.status = format!("Loaded {} vendors", app.vendors.len());
                }
                Err(e) => { app.status = format!("Error loading vendors: {}", e); }
            }
        }
        if matches!(app.current_tab(), Tab::Reports) && app.reports.is_empty() {
            match api.reports().await {
                Ok(items) => {
                    app.reports = items;
                    if !app.reports.is_empty() { app.report_state.select(Some(0)); app.report_page = 0; }
                    app.status = format!("Loaded {} reports", app.reports.len());
                }
                Err(e) => { app.status = format!("Error loading reports: {}", e); }
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
