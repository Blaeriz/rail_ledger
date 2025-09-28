pub mod batches;
pub mod vendors;
pub mod reports;
pub mod overview;
pub mod users;
pub mod tickets;
pub mod qr;
pub mod system;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;

use crate::app::{App, Tab};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
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

    // Main content by tab
    match app.current_tab() {
    Tab::Overview => overview::render(f, chunks[1], app),
        Tab::Batches => batches::render(f, chunks[1], app),
        Tab::Vendors => vendors::render(f, chunks[1], app),
        Tab::Reports => reports::render(f, chunks[1], app),
        Tab::Users => users::render(f, chunks[1], app),
        Tab::Tickets => tickets::render(f, chunks[1], app),
    Tab::QR => qr::render(f, chunks[1], app),
    Tab::System => system::render(f, chunks[1], app),
        other => draw_placeholder(f, chunks[1], other),
    }

    // Status bar with mini legend
    let legend = "Keys: q quit · Tab/Shift-Tab switch · ↑/↓ move · PgUp/PgDn jump · ←/→ page · Home/End first/last · M/H/D change metrics window";
    let status = Block::default()
        .borders(Borders::TOP)
        .title(format!("{}  ||  {}", legend, app.status));
    f.render_widget(status, chunks[2]);
}

fn draw_placeholder(f: &mut Frame, area: ratatui::prelude::Rect, tab: Tab) {
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
