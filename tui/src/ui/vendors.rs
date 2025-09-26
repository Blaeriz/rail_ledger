use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::{App, PAGE_SIZE};
use crate::models::Vendor;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_table(f, cols[0], app);
    draw_detail(f, cols[1], &app.vendors, app.vendor_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, app: &mut App) {
    let items = &app.vendors;
    let total = items.len();
    let total_pages = if total == 0 { 1 } else { ((total - 1) / PAGE_SIZE) + 1 };
    let page = app.vendor_page.min(total_pages.saturating_sub(1));
    app.vendor_page = page; // clamp
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let page_items = &items[start..end];

    let header = Row::new(vec!["VENDOR ID", "CITY", "STATE", "EMAIL", "BATCHES"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = page_items.iter().map(|v| {
        Row::new(vec![
            v.vendor_id.clone(),
            v.city.clone().unwrap_or_else(|| "-".into()),
            v.state.clone().unwrap_or_else(|| "-".into()),
            v.email.clone().unwrap_or_else(|| "-".into()),
            v.no_of_batches.map(|n| n.to_string()).unwrap_or_else(|| "-".into()),
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(16),
        Constraint::Length(14),
        Constraint::Length(10),
        Constraint::Length(24),
        Constraint::Length(9),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Vendors  (Page {}/{}, Rows {}-{} of {})",
        page + 1,
        total_pages,
        if total == 0 { 0 } else { start + 1 },
        end,
        total
    )))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    let mut local_state = TableState::default();
    if let Some(global_idx) = app.vendor_state.selected() {
        if global_idx >= start && global_idx < end {
            local_state.select(Some(global_idx - start));
        } else if start < end {
            local_state.select(None);
        }
    }

    f.render_stateful_widget(table, area, &mut local_state);
}

fn draw_detail(f: &mut Frame, area: Rect, items: &[Vendor], selected: Option<usize>) {
    let title = "Vendor Details";
    if let Some(idx) = selected.and_then(|i| items.get(i).map(|v| (i, v))) {
        let (_, v) = idx;
        let lines = vec![
            format!("Vendor ID: {}", v.vendor_id),
            format!("City/State: {}/{}", v.city.clone().unwrap_or_else(|| "-".into()), v.state.clone().unwrap_or_else(|| "-".into())),
            format!("Email: {}", v.email.clone().unwrap_or_else(|| "-".into())),
            format!("Phone: {}", v.phone_number.clone().unwrap_or_else(|| "-".into())),
            format!("GST: {}", v.gst_no.clone().unwrap_or_else(|| "-".into())),
            format!("PAN: {}", v.pan_number.clone().unwrap_or_else(|| "-".into())),
            format!("No. of Batches: {}", v.no_of_batches.map(|n| n.to_string()).unwrap_or_else(|| "-".into())),
            format!(
                "Audit Date: {}",
                v.audit_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "-".to_string())
            ),
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
