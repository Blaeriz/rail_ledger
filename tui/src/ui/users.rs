use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap};
use ratatui::Frame;

use crate::app::{App, PAGE_SIZE};
use crate::models::User;

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_table(f, cols[0], app);
    draw_detail(f, cols[1], &app.users, app.user_state.selected());
}

fn draw_table(f: &mut Frame, area: Rect, app: &mut App) {
    let items = &app.users;
    let total = items.len();
    let total_pages = if total == 0 { 1 } else { ((total - 1) / PAGE_SIZE) + 1 };
    let page = app.user_page.min(total_pages.saturating_sub(1));
    app.user_page = page;
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let page_items = &items[start..end];

    let header = Row::new(vec!["USER ID", "NAME", "ROLE", "PHONE"]) 
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    let rows = page_items.iter().map(|u| {
        Row::new(vec![
            u.user_id.clone(),
            u.name.clone().unwrap_or_else(|| "-".into()),
            u.user_role.clone().unwrap_or_else(|| "-".into()),
            u.phone_number.clone().unwrap_or_else(|| "-".into()),
        ])
    });

    let table = Table::new(rows, [
        Constraint::Length(16),
        Constraint::Length(18),
        Constraint::Length(12),
        Constraint::Length(16),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Users  (Page {}/{}, Rows {}-{} of {})",
        page + 1,
        total_pages,
        if total == 0 { 0 } else { start + 1 },
        end,
        total
    )))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    let mut local_state = TableState::default();
    if let Some(global_idx) = app.user_state.selected() {
        if global_idx >= start && global_idx < end {
            local_state.select(Some(global_idx - start));
        }
    }
    f.render_stateful_widget(table, area, &mut local_state);
}

fn draw_detail(f: &mut Frame, area: Rect, items: &[User], selected: Option<usize>) {
    let title = "User Details";
    if let Some(idx) = selected.and_then(|i| items.get(i).map(|u| (i, u))) {
        let (_, u) = idx;
        let lines = vec![
            format!("User ID: {}", u.user_id),
            format!("Name: {}", u.name.clone().unwrap_or_else(|| "-".into())),
            format!("Role: {}", u.user_role.clone().unwrap_or_else(|| "-".into())),
            format!("Phone: {}", u.phone_number.clone().unwrap_or_else(|| "-".into())),
            format!("Aadhar: {}", u.aadhar.clone().unwrap_or_else(|| "-".into())),
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
