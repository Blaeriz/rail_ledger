use std::io;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Tabs};
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
}

impl App {
    fn new() -> Self {
        Self { tab_index: 0, running: true, status: String::from("Press q to quit · Tab/Shift-Tab to switch tabs") }
    }

    fn current_tab(&self) -> Tab {
        Tab::all()[self.tab_index]
    }
}

fn ui(f: &mut Frame, app: &App) {
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

    // Content placeholder
    draw_content(f, chunks[1], app.current_tab());

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

#[tokio::main]
async fn main() -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let mut app = App::new();

    // Draw first frame
    terminal.draw(|f| ui(f, &app))?;

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
                        _ => {}
                    }
                }
            }
        }

        if !app.running { break; }

        terminal.draw(|f| ui(f, &app))?;
    }

    // Restore terminal
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
