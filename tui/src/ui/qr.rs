use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(area);

    draw_input(f, rows[0], app);
    draw_result(f, rows[1], app);
}

fn draw_input(f: &mut Frame, area: Rect, app: &App) {
    let mut title = String::from("QR Lookup: paste or type qr_hash and press Enter");
    if app.qr_input_focused {
        title.push_str("  [Focused]");
    } else {
        title.push_str("  [Press i to focus]");
    }
    let help = Line::from(vec![
        Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(": search    "),
        Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(": clear    "),
        Span::styled("i", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": focus input"),
    ]);
    let content = format!("{}\n{}", app.qr_input, "");
    let p = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .style(Style::default());
    f.render_widget(p, area);
    // Render help as an overlay line below input if there's space
    let help_area = Rect { x: area.x + 1, y: area.y + area.height.saturating_sub(1), width: area.width.saturating_sub(2), height: 1 };
    f.render_widget(Paragraph::new(help), help_area);
}

fn draw_result(f: &mut Frame, area: Rect, app: &App) {
    if let Some(err) = &app.qr_error {
        let p = Paragraph::new(err.as_str())
            .block(Block::default().borders(Borders::ALL).title("Result"))
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
        return;
    }

    if let Some(b) = &app.qr_result {
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
            format!("QR Hash: {}", b.qr_hash.clone().unwrap_or_else(|| "-".into())),
        ]
        .join("\n");
        let p = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Result"))
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
    } else {
        let p = Paragraph::new("Type a qr_hash above and press Enter")
            .block(Block::default().borders(Borders::ALL).title("Result"))
            .wrap(Wrap { trim: true });
        f.render_widget(p, area);
    }
}
