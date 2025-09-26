use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::{App, PAGE_SIZE};
use crate::models::Ticket;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    draw_table(f, cols[0], app);
    draw_detail(f, cols[1], &app.tickets, app.ticket_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, app: &mut App) {
    let items = &app.tickets;
    let total = items.len();
    let total_pages = if total == 0 { 1 } else { ((total - 1) / PAGE_SIZE) + 1 };
    let page = app.ticket_page.min(total_pages.saturating_sub(1));
    app.ticket_page = page;
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let page_items = &items[start..end];

    let header = Row::new(vec!["ID", "TITLE", "PRIORITY", "STATUS", "UPDATED"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = page_items.iter().map(|t| {
        Row::new(vec![
            t.ticket_id.clone(),
            t.title.clone(),
            t.priority.clone().unwrap_or_else(|| "-".into()),
            t.status.clone().unwrap_or_else(|| "-".into()),
            t.updated_at.clone().unwrap_or_else(|| t.created_at.clone().unwrap_or_else(|| "-".into())),
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(10),
        Constraint::Length(30),
        Constraint::Length(10),
        Constraint::Length(12),
        Constraint::Length(24),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Tickets  (Page {}/{}, Rows {}-{} of {})",
        page + 1,
        total_pages,
        if total == 0 { 0 } else { start + 1 },
        end,
        total
    )))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    let mut local_state = TableState::default();
    if let Some(global_idx) = app.ticket_state.selected() {
        if global_idx >= start && global_idx < end {
            local_state.select(Some(global_idx - start));
        }
    }
    f.render_stateful_widget(table, area, &mut local_state);
}

fn draw_detail(f: &mut Frame, area: Rect, items: &[Ticket], selected: Option<usize>) {
    let title = "Ticket Details";
    if let Some(idx) = selected.and_then(|i| items.get(i).map(|t| (i, t))) {
        let (_, t) = idx;
        let lines = vec![
            format!("Ticket ID: {}", t.ticket_id),
            format!("Title: {}", t.title),
            format!("Status: {}", t.status.clone().unwrap_or_else(|| "-".into())),
            format!("Priority: {}", t.priority.clone().unwrap_or_else(|| "-".into())),
            format!("Category: {}", t.category.clone().unwrap_or_else(|| "-".into())),
            format!("Created By: {}", t.created_by.clone().unwrap_or_else(|| "-".into())),
            format!("Assigned To: {}", t.assigned_to.clone().unwrap_or_else(|| "-".into())),
            format!("Created: {}", t.created_at.clone().unwrap_or_else(|| "-".into())),
            format!("Updated: {}", t.updated_at.clone().unwrap_or_else(|| "-".into())),
            format!("Resolved: {}", t.resolved_at.clone().unwrap_or_else(|| "-".into())),
            format!("Resolution Notes:\n{}", t.resolution_notes.clone().unwrap_or_else(|| "-".into())),
            format!("Attachments: {}", t.attachments.clone().map(|v| v.join(", ")).unwrap_or_else(|| "-".into())),
        ]
        .join("\n");

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
