use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::{App, PAGE_SIZE};
use crate::models::Batch;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    draw_table(f, cols[0], app);
    // Map selected (global) index into detail
    draw_detail(f, cols[1], &app.batches, app.batch_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, app: &mut App) {
    let items = &app.batches;
    let total = items.len();
    let total_pages = if total == 0 { 1 } else { ((total - 1) / PAGE_SIZE) + 1 };
    let page = app.batch_page.min(total_pages.saturating_sub(1));
    app.batch_page = page; // clamp
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let page_items = &items[start..end];

    let header = Row::new(vec!["BATCH ID", "VENDOR", "STATUS", "PROD DATE", "EXPIRY"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = page_items.iter().map(|b| {
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
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Batches  (Page {}/{}, Rows {}-{} of {})",
        page + 1,
        total_pages,
        if total == 0 { 0 } else { start + 1 },
        end,
        total
    )))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    // Translate global selection index onto this page's local index window.
    // If selection is None or outside current page, snap to first row of page when drawing.
    let mut local_state = TableState::default();
    if let Some(global_idx) = app.batch_state.selected() {
        if global_idx >= start && global_idx < end {
            local_state.select(Some(global_idx - start));
        } else if start < end {
            // keep selection consistent but outside page; do not select in local to avoid highlighting wrong row
            local_state.select(None);
        }
    }

    f.render_stateful_widget(table, area, &mut local_state);
    // After render, map local selection (if changed by keyboard handlers) back to global.
    // Note: keyboard handlers operate on global state; render does not modify app.batch_state.
}

fn draw_detail(f: &mut Frame, area: Rect, items: &[Batch], selected: Option<usize>) {
    let title = "Details";
    if let Some(idx) = selected.and_then(|i| items.get(i).map(|b| (i, b))) {
        let (_, b) = idx;
        let lines = vec![
            format!("Batch ID: {}", b.batch_id),
            format!("Vendor ID: {}", b.vendor_id.clone().unwrap_or_else(|| "-".into())),
            format!("Status: {}", b.qc_status.clone().unwrap_or_else(|| "-".into())),
            format!(
                "Production: {}",
                b.date_of_production
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "-".to_string())
            ),
            format!(
                "Expiry: {}",
                b.expiry_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "-".to_string())
            ),
            format!(
                "Last Inspection: {}",
                b.last_inspection_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "-".to_string())
            ),
            format!(
                "Fitment: {} at {}",
                b.fitment_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "-".to_string()),
                b.fitment_location.clone().unwrap_or_else(|| "-".into())
            ),
            format!("QR Hash: {}", b.qr_hash.clone().unwrap_or_else(|| "-".into())),
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
