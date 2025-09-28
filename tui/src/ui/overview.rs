use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
use ratatui::symbols;
use ratatui::text::{Line as TuiLine, Span};

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
    // Datasets will be created later for the selected route
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

    // Build per-route, per-method series (segregate GET/POST) while keeping same color per route
    let mut series_storage: Vec<(String, String, Vec<(f64, f64)>)> = Vec::new();
    for (route, methods) in &app.live_metrics {
        for (method, timestamps) in methods {
            use std::collections::HashMap;
            let mut freq: HashMap<&str, u32> = HashMap::new();
            for ts in timestamps {
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
                series_storage.push((route.clone(), method.clone(), points));
            }
        }
    }

    // Sort for stable ordering across renders
    series_storage.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // Gather unique routes for selection/legend panel
    let mut unique_routes: Vec<String> = series_storage.iter().map(|(r, _, _)| r.clone()).collect();
    unique_routes.sort();
    unique_routes.dedup();

    // Determine unique routes and selected route index
    if series_storage.is_empty() {
        let p = Paragraph::new("No live metrics yet…")
            .block(Block::default().borders(Borders::ALL).title("Live API metrics"));
        f.render_widget(p, area);
        return;
    }

    // Round max_y up to nearest 10 and build y-axis labels at steps of 10
    let max_y_u = max_y.ceil() as u32;
    let mut y_upper = if max_y_u == 0 { 10 } else { ((max_y_u + 9) / 10) * 10 };
    if y_upper < 10 { y_upper = 10; }
    let y_labels: Vec<_> = (0..=y_upper).step_by(10).map(|n| n.to_string().into()).collect();

    // Split area into chart (left) and a larger legend panel (right/top-right)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        // 80/20 split per request
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);

    // If we have multiple routes, show only the selected route's datasets on the graph
    let mut selected_route: Option<String> = None;
    if !unique_routes.is_empty() {
        let sel = app.metrics_route_index.min(unique_routes.len() - 1);
        selected_route = Some(unique_routes[sel].clone());
    }
    // Build datasets only for the selected route; color by METHOD (GET: green, POST: blue)
    let mut filtered: Vec<Dataset> = Vec::new();
    if let Some(ref route) = selected_route {
        for (r, method, points) in &series_storage {
            if r == route {
                let color = match method.as_str() {
                    "GET" => Color::Green,
                    "POST" => Color::Blue,
                    _ => Color::White,
                };
                let ds = Dataset::default()
                    .name(format!("{} {}", method, r))
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(color))
                    .graph_type(GraphType::Line)
                    .data(points);
                filtered.push(ds);
            }
        }
    }
    let chart = Chart::new(filtered)
        .block(
            Block::bordered().title(
                TuiLine::from(format!(
                    "API requests (per {}) — {}",
                    app.metrics_scale.label(),
                    selected_route.clone().unwrap_or_else(|| "all routes".to_string())
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
                .bounds([0.0, y_upper as f64])
                .labels(y_labels),
        );
    f.render_widget(chart, cols[0]);

    // Build custom legend panel: method key + route list (no route colorcoding)
    let mut legend_lines: Vec<TuiLine> = Vec::new();
    // Method key
    legend_lines.push(TuiLine::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::raw("GET"),
    ]));
    legend_lines.push(TuiLine::from(vec![
        Span::styled("● ", Style::default().fg(Color::Blue)),
        Span::raw("POST"),
    ]));
    legend_lines.push(TuiLine::from(""));
    // Right panel legend: show only routes, highlight the selected one
    for route in &unique_routes {
        let mut line = vec![Span::styled("• ", Style::default().fg(Color::Gray)), Span::raw(route.clone())];
        if Some(route) == selected_route.as_ref() { for s in &mut line { *s = s.clone().bold(); } }
        legend_lines.push(TuiLine::from(line));
    }

    let legend_par = Paragraph::new(legend_lines)
        .block(Block::default().borders(Borders::ALL).title("Legend"))
        .wrap(Wrap { trim: true });
    f.render_widget(legend_par, cols[1]);
}

// Removed route-based color palette: legend now uses only method colors (GET/POST),
// and datasets are colored by method as well.
