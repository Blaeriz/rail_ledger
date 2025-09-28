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

    // Generate unique colors per endpoint (route) for current set
    use std::collections::HashMap as Map;
    let mut unique_routes: Vec<String> = series_storage.iter().map(|(r, _, _)| r.clone()).collect();
    unique_routes.sort();
    unique_routes.dedup();
    let palette = gen_palette(unique_routes.len());
    let mut route_color: Map<String, Color> = Map::new();
    for (idx, r) in unique_routes.iter().enumerate() {
        route_color.insert(r.clone(), palette[idx]);
    }

    for (route, method, points) in &series_storage {
        let color = route_color.get(route).cloned().unwrap_or(Color::White);
        let ds = Dataset::default()
            .name(format!("{} {}", method, route))
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(color))
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

    // Round max_y up to nearest 10 and build y-axis labels at steps of 10
    let max_y_u = max_y.ceil() as u32;
    let mut y_upper = if max_y_u == 0 { 10 } else { ((max_y_u + 9) / 10) * 10 };
    if y_upper < 10 { y_upper = 10; }
    let y_labels: Vec<_> = (0..=y_upper).step_by(10).map(|n| n.to_string().into()).collect();

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
                .bounds([0.0, y_upper as f64])
                .labels(y_labels),
        );

    // Split area into chart (left) and a larger legend panel (right/top-right)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        // Reduce legend panel width by 75%: from 40% -> 10%
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    f.render_widget(chart, cols[0]);

    // Build custom legend lines (colored bullet + route name)
    let mut legend_lines: Vec<TuiLine> = Vec::new();
    for (route, method, _points) in &series_storage {
        let color = route_color.get(route).cloned().unwrap_or(Color::White);
        legend_lines.push(TuiLine::from(vec![
            Span::styled("● ", Style::default().fg(color)),
            Span::raw(format!("{} {}", method, route)),
        ]));
    }

    let legend_par = Paragraph::new(legend_lines)
        .block(Block::default().borders(Borders::ALL).title("Legend"))
        .wrap(Wrap { trim: true });
    f.render_widget(legend_par, cols[1]);
}

// Generate N visually distinct colors by sweeping hue; fixed saturation/lightness.
fn gen_palette(n: usize) -> Vec<Color> {
    if n == 0 { return Vec::new(); }
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let h = (i as f32) / (n as f32); // 0..1
        let (r, g, b) = hsl_to_rgb(h, 0.65, 0.5);
        v.push(Color::Rgb(r, g, b));
    }
    v
}

// Minimal HSL->RGB converter; h,s,l in 0..1
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let a = s * f32::min(l, 1.0 - l);
    fn f(n: f32, h: f32, l: f32, a: f32) -> u8 {
        let k = (n + h * 12.0) % 12.0;
        let color = l - a * f32::max(f32::min(f32::min(k - 3.0, 9.0 - k), 1.0), -1.0);
        (color.clamp(0.0, 1.0) * 255.0).round() as u8
    }
    (f(0.0, h, l, a), f(8.0/12.0, h, l, a), f(4.0/12.0, h, l, a))
}
