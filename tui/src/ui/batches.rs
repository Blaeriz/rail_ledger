use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::models::Batch;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    draw_table(f, cols[0], &app.batches, &mut app.batch_state);
    draw_detail(f, cols[1], &app.batches, app.batch_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, items: &[Batch], state: &mut TableState) {
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
