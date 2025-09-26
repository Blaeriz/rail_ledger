use std::io;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
mod api;
mod config;
mod models;

use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Row, Table, TableState, Tabs};
use ratatui::Terminal;

#[derive(Copy, Clone, Debug)]
enum Tab {
    Overview,
    Batches,
    Vendors,
    Reports,
    Users,
    Tickets,
    AI,
    QR,
    System,
}

impl Tab {
    fn all() -> &'static [Tab] {
        use Tab::*;
        &[Overview, Batches, Vendors, Reports, Users, Tickets, AI, QR, System]
    }

    fn title(self) -> &'static str {
        use Tab::*;
        match self {
            Overview => "Overview",
            Batches => "Batches",
            Vendors => "Vendors",
            Reports => "Reports",
            Users => "Users",
            Tickets => "Tickets",
            AI => "AI",
            QR => "QR",
            System => "System",
        }
    }
}

struct App {
    tab_index: usize,
    running: bool,
    status: String,
    // Data
    batches: Vec<models::Batch>,
    batch_state: TableState,
}

impl App {
    fn new() -> Self {
        Self {
            tab_index: 0,
            running: true,
            status: String::from("Press q to quit · Tab/Shift-Tab to switch tabs"),
            batches: Vec::new(),
            batch_state: TableState::default(),
        }
    }

    fn current_tab(&self) -> Tab {
        Tab::all()[self.tab_index]
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // tabs
            Constraint::Min(1),    // content
            Constraint::Length(1), // status
        ])
        .split(f.size());

    // Tabs
    let titles: Vec<Line> = Tab::all()
        .iter()
        .map(|t| Line::from(Span::styled(t.title(), Style::default().fg(Color::Cyan))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Rail Ledger TUI"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .select(app.tab_index);

    f.render_widget(tabs, chunks[0]);

    // Content
    match app.current_tab() {
        Tab::Batches => draw_batches_table(f, chunks[1], &app.batches, &mut app.batch_state),
        other => draw_content(f, chunks[1], other),
    }

    // Status bar
    let status = Block::default().borders(Borders::TOP).title(app.status.as_str());
    f.render_widget(status, chunks[2]);
}

fn draw_content(f: &mut Frame, area: Rect, tab: Tab) {
    let title = match tab {
        Tab::Overview => "Overview: stats and recent activity (read-only)",
        Tab::Batches => "Batches: list, filters, detail",
        Tab::Vendors => "Vendors: list, metrics, detail",
        Tab::Reports => "Reports: inspection list + detail",
        Tab::Users => "Users: list + detail",
        Tab::Tickets => "Tickets: list + detail (mock)",
        Tab::AI => "AI Insights: summaries and recommendations",
        Tab::QR => "QR Tools: lookup by qr_hash",
        Tab::System => "System: config and version",
    };

    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(block, area);
}

fn draw_batches_table(f: &mut Frame, area: Rect, items: &[models::Batch], state: &mut TableState) {
    let header = Row::new(vec!["BATCH ID", "VENDOR", "STATUS", "PROD DATE", "EXPIRY"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = items.iter().map(|b| {
        let prod = b
            .date_of_production
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".to_string());
        let exp = b
            .expiry_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".to_string());
        Row::new(vec![
            b.batch_id.clone(),
            b.vendor_id.clone().unwrap_or_else(|| "-".into()),
            b.qc_status.clone().unwrap_or_else(|| "-".into()),
            prod,
            exp,
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(16),
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(12),
        Constraint::Length(12),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Batches"))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    f.render_stateful_widget(table, area, state);
}

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
    terminal.draw(|f| ui(f, &mut app))?;

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

        terminal.draw(|f| ui(f, &mut app))?;
    }

    // Restore terminal
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
