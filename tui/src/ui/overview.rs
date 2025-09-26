use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, Wrap};
use ratatui::Frame;

use crate::app::App;
use chrono::{DateTime, NaiveDateTime, Utc};

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
    // Sort by created_at desc using robust parsing; fall back to original order
    let mut items: Vec<_> = app.reports.iter().collect();
    items.sort_by(|a, b| {
        let ad = parse_created_at(a.created_at.as_deref());
        let bd = parse_created_at(b.created_at.as_deref());
        bd.cmp(&ad) // desc
    });
    let rows = items
        .into_iter()
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

fn parse_created_at(s: Option<&str>) -> Option<DateTime<Utc>> {
    let s = s?;
    // Try RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    // Common fallback: "YYYY-mm-dd HH:MM:SS"
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc));
    }
    // Another common: without seconds fraction but with T
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc));
    }
    None
}
