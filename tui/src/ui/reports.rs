use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::{App, PAGE_SIZE};
use crate::models::Report;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    draw_table(f, cols[0], app);
    draw_detail(f, cols[1], &app.reports, app.report_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, app: &mut App) {
    let items = &app.reports;
    let total = items.len();
    let total_pages = if total == 0 { 1 } else { ((total - 1) / PAGE_SIZE) + 1 };
    let page = app.report_page.min(total_pages.saturating_sub(1));
    app.report_page = page; // clamp
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let page_items = &items[start..end];

    let header = Row::new(vec!["ID", "BATCH", "INSPECTOR", "STATUS", "CREATED"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = page_items.iter().map(|r| {
        Row::new(vec![
            r.report_id.to_string(),
            r.batch_id.clone(),
            r.inspector_name.clone(),
            r.status.map(|s| s.to_string()).unwrap_or_else(|| "-".into()),
            r.created_at.clone().unwrap_or_else(|| "-".into()),
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(6),
        Constraint::Length(16),
        Constraint::Length(18),
        Constraint::Length(8),
        Constraint::Length(26),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Reports  (Page {}/{}, Rows {}-{} of {})",
        page + 1,
        total_pages,
        if total == 0 { 0 } else { start + 1 },
        end,
        total
    )))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    let mut local_state = TableState::default();
    if let Some(global_idx) = app.report_state.selected() {
        if global_idx >= start && global_idx < end {
            local_state.select(Some(global_idx - start));
        } else if start < end {
            local_state.select(None);
        }
    }

    f.render_stateful_widget(table, area, &mut local_state);
}

fn draw_detail(f: &mut Frame, area: Rect, items: &[Report], selected: Option<usize>) {
    let title = "Report Details";
    if let Some(idx) = selected.and_then(|i| items.get(i).map(|r| (i, r))) {
        let (_, r) = idx;
        let lines = vec![
            format!("Report ID: {}", r.report_id),
            format!("Batch ID: {}", r.batch_id),
            format!("Inspector: {}", r.inspector_name),
            format!("Status: {}", r.status.map(|s| s.to_string()).unwrap_or_else(|| "-".into())),
            format!("Created: {}", r.created_at.clone().unwrap_or_else(|| "-".into())),
            format!("Remark:\n{}", r.remark.clone().unwrap_or_else(|| "-".into())),
        ].join("\n");

        let p = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
    } else {
        let p = Paragraph::new("No selection")
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
    }
}
