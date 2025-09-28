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

use app::{App, Tab, PAGE_SIZE, MetricsScale};

#[tokio::main]
async fn main() -> Result<()> {
    // Config + API
    let cfg = config::Config::default();
    let refresh_interval = cfg.refresh_interval; // capture for periodic refresh
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
                        KeyCode::Char('M') => {
                            // minutes view toggle: cycle 5 -> 15 -> 60
                            app.metrics_scale = match app.metrics_scale {
                                MetricsScale::Minutes(5) => MetricsScale::Minutes(15),
                                MetricsScale::Minutes(15) => MetricsScale::Minutes(60),
                                MetricsScale::Minutes(_) => MetricsScale::Minutes(5),
                                MetricsScale::Hours(h) => MetricsScale::Minutes(h.saturating_mul(60)),
                                MetricsScale::Days(d) => MetricsScale::Minutes(d.saturating_mul(1440)),
                            };
                            app.status = format!("Metrics window: {} mins", app.metrics_scale.to_minutes());
                        }
                        KeyCode::Char('H') => {
                            // hours view preset: 1h, 6h, 24h
                            app.metrics_scale = match app.metrics_scale {
                                MetricsScale::Hours(1) => MetricsScale::Hours(6),
                                MetricsScale::Hours(6) => MetricsScale::Hours(24),
                                MetricsScale::Hours(_) => MetricsScale::Hours(1),
                                MetricsScale::Minutes(m) => MetricsScale::Hours((m + 59) / 60),
                                MetricsScale::Days(d) => MetricsScale::Hours(d.saturating_mul(24)),
                            };
                            app.status = format!("Metrics window: {} mins", app.metrics_scale.to_minutes());
                        }
                        KeyCode::Char('D') => {
                            // days view preset: 1d, 3d, 7d
                            app.metrics_scale = match app.metrics_scale {
                                MetricsScale::Days(1) => MetricsScale::Days(3),
                                MetricsScale::Days(3) => MetricsScale::Days(7),
                                MetricsScale::Days(_) => MetricsScale::Days(1),
                                MetricsScale::Hours(h) => MetricsScale::Days((h + 23) / 24),
                                MetricsScale::Minutes(m) => MetricsScale::Days((m + 1439) / 1440),
                            };
                            app.status = format!("Metrics window: {} mins", app.metrics_scale.to_minutes());
                        }
                        KeyCode::Char('i') => {
                            if matches!(app.current_tab(), Tab::QR) {
                                app.qr_input_focused = true;
                                app.status = "QR input focused".into();
                            }
                        }
                        KeyCode::Esc => {
                            if matches!(app.current_tab(), Tab::QR) {
                                app.qr_input.clear();
                                app.qr_result = None;
                                app.qr_error = None;
                                app.qr_input_focused = true;
                                app.status = "Cleared QR search".into();
                            }
                        }
                        KeyCode::Enter => {
                            if matches!(app.current_tab(), Tab::QR) && !app.qr_input.trim().is_empty() {
                                let query = app.qr_input.trim().to_string();
                                match api.batch_by_qr_hash(&query).await {
                                    Ok(batch) => { app.qr_result = Some(batch); app.qr_error = None; app.status = "QR match loaded".into(); }
                                    Err(e) => { app.qr_result = None; app.qr_error = Some(format!("{}", e)); app.status = "QR lookup failed".into(); }
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            match app.current_tab() {
                                Tab::Overview => {
                                    if !app.live_metrics.is_empty() {
                                        let mut routes: Vec<String> = app.live_metrics.keys().cloned().collect();
                                        routes.sort();
                                        if !routes.is_empty() {
                                            if app.metrics_route_index > 0 {
                                                app.metrics_route_index -= 1;
                                            } else {
                                                app.metrics_route_index = routes.len() - 1;
                                            }
                                            app.status = format!("Selected: {}", routes[app.metrics_route_index]);
                                        }
                                    }
                                }
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        let idx = app.user_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(1);
                                        app.user_state.select(Some(new_idx));
                                        app.user_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        let idx = app.ticket_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(1);
                                        app.ticket_state.select(Some(new_idx));
                                        app.ticket_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            match app.current_tab() {
                                Tab::Overview => {
                                    if !app.live_metrics.is_empty() {
                                        let mut routes: Vec<String> = app.live_metrics.keys().cloned().collect();
                                        routes.sort();
                                        if !routes.is_empty() {
                                            app.metrics_route_index = (app.metrics_route_index + 1) % routes.len();
                                            app.status = format!("Selected: {}", routes[app.metrics_route_index]);
                                        }
                                    }
                                }
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        let idx = app.user_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 1).min(len.saturating_sub(1));
                                        app.user_state.select(Some(new_idx));
                                        app.user_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        let idx = app.ticket_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 1).min(len.saturating_sub(1));
                                        app.ticket_state.select(Some(new_idx));
                                        app.ticket_page = new_idx / PAGE_SIZE;
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        let idx = app.user_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(10);
                                        app.user_state.select(Some(new_idx));
                                        app.user_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        let idx = app.ticket_state.selected().unwrap_or(0);
                                        let new_idx = idx.saturating_sub(10);
                                        app.ticket_state.select(Some(new_idx));
                                        app.ticket_page = new_idx / PAGE_SIZE;
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        let idx = app.user_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 10).min(len.saturating_sub(1));
                                        app.user_state.select(Some(new_idx));
                                        app.user_page = new_idx / PAGE_SIZE;
                                        app.status = format!("Row {}/{}", new_idx + 1, len);
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        let idx = app.ticket_state.selected().unwrap_or(0);
                                        let new_idx = (idx + 10).min(len.saturating_sub(1));
                                        app.ticket_state.select(Some(new_idx));
                                        app.ticket_page = new_idx / PAGE_SIZE;
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        if app.user_page > 0 {
                                            app.user_page -= 1;
                                            let new_idx = app.user_page * PAGE_SIZE;
                                            let new_idx = new_idx.min(len.saturating_sub(1));
                                            app.user_state.select(Some(new_idx));
                                            let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                            app.status = format!("Page {}/{} · Row {}/{}", app.user_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        if app.ticket_page > 0 {
                                            app.ticket_page -= 1;
                                            let new_idx = app.ticket_page * PAGE_SIZE;
                                            let new_idx = new_idx.min(len.saturating_sub(1));
                                            app.ticket_state.select(Some(new_idx));
                                            let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                            app.status = format!("Page {}/{} · Row {}/{}", app.ticket_page + 1, total_pages, new_idx + 1, len);
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 {
                                        let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                        if app.user_page + 1 < total_pages {
                                            app.user_page += 1;
                                            let mut new_idx = app.user_page * PAGE_SIZE;
                                            new_idx = new_idx.min(len.saturating_sub(1));
                                            app.user_state.select(Some(new_idx));
                                            app.status = format!("Page {}/{} · Row {}/{}", app.user_page + 1, total_pages, new_idx + 1, len);
                                        }
                                    }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 {
                                        let total_pages = ((len.saturating_sub(1)) / PAGE_SIZE) + 1;
                                        if app.ticket_page + 1 < total_pages {
                                            app.ticket_page += 1;
                                            let mut new_idx = app.ticket_page * PAGE_SIZE;
                                            new_idx = new_idx.min(len.saturating_sub(1));
                                            app.ticket_state.select(Some(new_idx));
                                            app.status = format!("Page {}/{} · Row {}/{}", app.ticket_page + 1, total_pages, new_idx + 1, len);
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 { app.user_state.select(Some(0)); app.user_page = 0; app.status = format!("Row {}/{}", 1, len); }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 { app.ticket_state.select(Some(0)); app.ticket_page = 0; app.status = format!("Row {}/{}", 1, len); }
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
                                Tab::Users => {
                                    let len = app.users.len();
                                    if len > 0 { let last = len - 1; app.user_state.select(Some(last)); app.user_page = last / PAGE_SIZE; app.status = format!("Row {}/{}", last + 1, len); }
                                }
                                Tab::Tickets => {
                                    let len = app.tickets.len();
                                    if len > 0 { let last = len - 1; app.ticket_state.select(Some(last)); app.ticket_page = last / PAGE_SIZE; app.status = format!("Row {}/{}", last + 1, len); }
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            // Text input for QR tab when focused
                            if matches!(app.current_tab(), Tab::QR) && app.qr_input_focused {
                                match key.code {
                                    KeyCode::Char(c) => { app.qr_input.push(c); }
                                    KeyCode::Backspace => { app.qr_input.pop(); }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        if !app.running { break; }

        // Lazy-load per tab (can be extended to timed refresh later)
    // Lazy-load per tab and periodic refresh
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
            // Load reports for stats
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

            // Initial metrics load
            if app.live_metrics.is_empty() {
                let mins = app.metrics_scale.to_minutes();
                match api.live_metrics(mins).await {
                    Ok(map) => {
                        app.live_metrics = map;
                        app.status = format!("Loaded live metrics ({} mins)", mins);
                    }
                    Err(e) => { app.status = format!("Error loading metrics: {}", e); }
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
        if matches!(app.current_tab(), Tab::Users) && app.users.is_empty() {
            match api.users().await {
                Ok(items) => {
                    app.users = items;
                    if !app.users.is_empty() { app.user_state.select(Some(0)); app.user_page = 0; }
                    app.status = format!("Loaded {} users", app.users.len());
                }
                Err(e) => { app.status = format!("Error loading users: {}", e); }
            }
        }
        if matches!(app.current_tab(), Tab::Tickets) && app.tickets.is_empty() {
            match api.tickets().await {
                Ok(items) => {
                    app.tickets = items;
                    if !app.tickets.is_empty() { app.ticket_state.select(Some(0)); app.ticket_page = 0; }
                    app.status = format!("Loaded {} tickets", app.tickets.len());
                }
                Err(e) => { app.status = format!("Error loading tickets: {}", e); }
            }
        }

        // Periodic refresh of the active tab list only
        let now = std::time::Instant::now();
    if now.duration_since(app.last_refresh) >= refresh_interval {
            match app.current_tab() {
                Tab::Batches => {
                    if let Ok(items) = api.batches().await {
                        let prev_len = app.batches.len();
                        app.batches = items;
                        if app.batches.is_empty() {
                            app.batch_state.select(None);
                            app.batch_page = 0;
                        } else if app.batch_state.selected().is_none() {
                            app.batch_state.select(Some(0));
                            app.batch_page = 0;
                        } else {
                            // keep selection in range
                            let sel = app.batch_state.selected().unwrap();
                            let new_sel = sel.min(app.batches.len().saturating_sub(1));
                            app.batch_state.select(Some(new_sel));
                            app.batch_page = new_sel / PAGE_SIZE;
                        }
                        if app.batches.len() != prev_len {
                            app.status = format!("Refreshed batches: {} items", app.batches.len());
                        }
                    }
                    app.last_refresh = now;
                }
                Tab::Overview => {
                    // Refresh live metrics more frequently than other lists
                    if now.duration_since(app.last_metrics_refresh) >= std::time::Duration::from_secs(5) {
                        let mins = app.metrics_scale.to_minutes();
                        if let Ok(map) = api.live_metrics(mins).await {
                            app.live_metrics = map;
                            // update ordered routes cache and clamp selection index
                            let mut routes: Vec<String> = app.live_metrics.keys().cloned().collect();
                            routes.sort();
                            app.metrics_routes = routes;
                            if !app.metrics_routes.is_empty() {
                                app.metrics_route_index = app.metrics_route_index.min(app.metrics_routes.len() - 1);
                            } else {
                                app.metrics_route_index = 0;
                            }
                            app.last_metrics_refresh = now;
                        }
                    }
                    app.last_refresh = now;
                }
                Tab::Vendors => {
                    if let Ok(items) = api.vendors().await {
                        let prev_len = app.vendors.len();
                        app.vendors = items;
                        if app.vendors.is_empty() {
                            app.vendor_state.select(None);
                            app.vendor_page = 0;
                        } else if app.vendor_state.selected().is_none() {
                            app.vendor_state.select(Some(0));
                            app.vendor_page = 0;
                        } else {
                            let sel = app.vendor_state.selected().unwrap();
                            let new_sel = sel.min(app.vendors.len().saturating_sub(1));
                            app.vendor_state.select(Some(new_sel));
                            app.vendor_page = new_sel / PAGE_SIZE;
                        }
                        if app.vendors.len() != prev_len {
                            app.status = format!("Refreshed vendors: {} items", app.vendors.len());
                        }
                    }
                    app.last_refresh = now;
                }
                Tab::Reports => {
                    if let Ok(items) = api.reports().await {
                        let prev_len = app.reports.len();
                        app.reports = items;
                        if app.reports.is_empty() {
                            app.report_state.select(None);
                            app.report_page = 0;
                        } else if app.report_state.selected().is_none() {
                            app.report_state.select(Some(0));
                            app.report_page = 0;
                        } else {
                            let sel = app.report_state.selected().unwrap();
                            let new_sel = sel.min(app.reports.len().saturating_sub(1));
                            app.report_state.select(Some(new_sel));
                            app.report_page = new_sel / PAGE_SIZE;
                        }
                        if app.reports.len() != prev_len {
                            app.status = format!("Refreshed reports: {} items", app.reports.len());
                        }
                    }
                    app.last_refresh = now;
                }
                Tab::Users => {
                    if let Ok(items) = api.users().await {
                        let prev_len = app.users.len();
                        app.users = items;
                        if app.users.is_empty() {
                            app.user_state.select(None);
                            app.user_page = 0;
                        } else if app.user_state.selected().is_none() {
                            app.user_state.select(Some(0));
                            app.user_page = 0;
                        } else {
                            let sel = app.user_state.selected().unwrap();
                            let new_sel = sel.min(app.users.len().saturating_sub(1));
                            app.user_state.select(Some(new_sel));
                            app.user_page = new_sel / PAGE_SIZE;
                        }
                        if app.users.len() != prev_len {
                            app.status = format!("Refreshed users: {} items", app.users.len());
                        }
                    }
                    app.last_refresh = now;
                }
                Tab::Tickets => {
                    if let Ok(items) = api.tickets().await {
                        let prev_len = app.tickets.len();
                        app.tickets = items;
                        if app.tickets.is_empty() {
                            app.ticket_state.select(None);
                            app.ticket_page = 0;
                        } else if app.ticket_state.selected().is_none() {
                            app.ticket_state.select(Some(0));
                            app.ticket_page = 0;
                        } else {
                            let sel = app.ticket_state.selected().unwrap();
                            let new_sel = sel.min(app.tickets.len().saturating_sub(1));
                            app.ticket_state.select(Some(new_sel));
                            app.ticket_page = new_sel / PAGE_SIZE;
                        }
                        if app.tickets.len() != prev_len {
                            app.status = format!("Refreshed tickets: {} items", app.tickets.len());
                        }
                    }
                    app.last_refresh = now;
                }
                _ => {}
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
