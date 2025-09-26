use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, Wrap};
use ratatui::Frame;

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(1)])
        .split(area);

    draw_stats(f, rows[0], app);
    draw_recent_reports(f, rows[1], app);
}

fn draw_stats(f: &mut Frame, area: Rect, app: &App) {
    let vendors = app.vendors.len();
    let batches = app.batches.len();
    let reports = app.reports.len();
    let text = format!(
        "Totals\n- Vendors: {}\n- Batches: {}\n- Reports: {}",
        vendors, batches, reports
    );
    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Overview"))
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn draw_recent_reports(f: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec!["ID", "BATCH", "INSPECTOR", "STATUS", "CREATED"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    // Take first 10 reports as a simple "recent" list (backend not sorted here)
    let rows = app
        .reports
        .iter()
        .take(10)
        .map(|r| Row::new(vec![
            r.report_id.to_string(),
            r.batch_id.clone(),
            r.inspector_name.clone(),
            r.status.map(|s| s.to_string()).unwrap_or_else(|| "-".into()),
            r.created_at.clone().unwrap_or_else(|| "-".into()),
        ]));

    let table = Table::new(rows, [
        Constraint::Length(6),
        Constraint::Length(16),
        Constraint::Length(18),
        Constraint::Length(8),
        Constraint::Length(26),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Recent Reports (first 10)"))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(table, area);
}
