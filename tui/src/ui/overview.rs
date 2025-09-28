use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType, LegendPosition};
use ratatui::symbols;
use ratatui::text::Line as TuiLine;

use crate::app::App;
use chrono::{Local, TimeDelta};

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(10),
        ])
        .split(area);

    draw_stats(f, rows[0], app);
    draw_live_metrics_chart(f, rows[1], app);
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

// Live metrics chart

fn draw_live_metrics_chart(f: &mut Frame, area: Rect, app: &App) {
    // Build datasets: one per endpoint with counts per minute bucket across the selected window
    let mut datasets: Vec<Dataset> = Vec::new();
    let mut max_x = 1.0f64;
    let mut max_y = 1.0f64;
    let minutes = app.metrics_scale.to_minutes().max(1);

    // Build the minute keys for the window [now - minutes + 1 .. now]
    let now = Local::now();
    let mut minute_keys: Vec<String> = Vec::with_capacity(minutes as usize);
    for i in (0..minutes).rev() {
        let dt = now - TimeDelta::minutes(i as i64);
        minute_keys.push(dt.format("%d%m%y%H%M").to_string());
    }

    let mut series_storage: Vec<(String, Vec<(f64, f64)>)> = Vec::new();
    for (route, entry) in &app.live_metrics {
        use std::collections::HashMap;
        let mut freq: HashMap<&str, u32> = HashMap::new();
        for ts in &entry.timestamps {
            *freq.entry(ts.as_str()).or_insert(0) += 1;
        }
        let mut points: Vec<(f64, f64)> = Vec::with_capacity(minute_keys.len());
        for (idx, key) in minute_keys.iter().enumerate() {
            let y = *freq.get(key.as_str()).unwrap_or(&0) as f64;
            points.push((idx as f64, y));
        }
        if !points.is_empty() {
            max_x = f64::max(max_x, points.last().unwrap().0);
            let local_max_y = points.iter().fold(0.0f64, |m, &(_, y)| f64::max(m, y));
            max_y = max_y.max(local_max_y);
            series_storage.push((route.clone(), points));
        }
    }

    for (route, points) in &series_storage {
        let ds = Dataset::default()
            .name(route.clone())
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
            .data(points);
        datasets.push(ds);
    }

    if datasets.is_empty() {
        let p = Paragraph::new("No live metrics yet…")
            .block(Block::default().borders(Borders::ALL).title("Live API metrics"));
        f.render_widget(p, area);
        return;
    }

    let chart = Chart::new(datasets)
        .block(
            Block::bordered().title(
                TuiLine::from(format!(
                    "API requests (per {}) — endpoints",
                    app.metrics_scale.label()
                ))
                .cyan()
                .bold()
                .centered(),
            ),
        )
        .x_axis(
            Axis::default()
                .title("time buckets")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, (max_x + 1.0).max(5.0)])
                .labels(vec!["0".bold(), "".into(), format!("{:.0}", max_x.max(1.0)).bold().into()]),
        )
        .y_axis(
            Axis::default()
                .title("requests")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, (max_y + 1.0).max(5.0)])
                .labels(vec!["0".bold(), "".into(), format!("{:.0}", max_y.max(1.0)).bold().into()]),
        )
        .legend_position(Some(LegendPosition::TopLeft));

    f.render_widget(chart, area);
}
