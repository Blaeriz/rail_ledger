use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

use crate::app::App;

pub fn render(f: &mut Frame, area: Rect, _app: &mut App) {
    // Collect system metrics (Linux)
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_cpu();
    sys.refresh_memory();

    // CPU
    let global_cpu = sys.global_cpu_info();
    let cpu_usage = global_cpu.cpu_usage();
    let cpus = sys.cpus();
    let avg_per_core: f32 = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / (cpus.len() as f32)
    };

    // Memory
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    // Networks (sum over interfaces)
    let networks = Networks::new_with_refreshed_list();
    let mut rx_total: u64 = 0;
    let mut tx_total: u64 = 0;
    for (_name, data) in networks.iter() {
        rx_total = rx_total.saturating_add(data.total_received());
        tx_total = tx_total.saturating_add(data.total_transmitted());
    }

    // Disks
    let disks = Disks::new_with_refreshed_list();
    let mut disk_lines: Vec<String> = Vec::new();
    for d in &disks {
        let total = d.total_space();
        let avail = d.available_space();
        let used = total.saturating_sub(avail);
        let name = d.name().to_string_lossy();
        disk_lines.push(format!("- {}: used {} / {} ({} free)", name, fmt_bytes(used), fmt_bytes(total), fmt_bytes(avail)));
    }

    let text = format!(
        "System metrics (Linux)\n\
         CPU: global {:.1}% · per-core avg {:.1}% (cores: {})\n\
         Memory: {} / {} used · Swap: {} / {} used\n\
         Network: RX {} · TX {} (cumulative)\n\
         Disks:\n{}",
        cpu_usage,
        avg_per_core,
        cpus.len(),
        fmt_bytes(used_mem * 1024),
        fmt_bytes(total_mem * 1024),
        fmt_bytes(used_swap * 1024),
        fmt_bytes(total_swap * 1024),
        fmt_bytes(rx_total),
        fmt_bytes(tx_total),
        disk_lines.join("\n")
    );

    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("System"))
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);
}

fn fmt_bytes(b: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut val = b as f64;
    let mut idx = 0;
    while val >= 1024.0 && idx < UNITS.len() - 1 {
        val /= 1024.0;
        idx += 1;
    }
    if idx == 0 { format!("{} {}", b, UNITS[idx]) } else { format!("{:.1} {}", val, UNITS[idx]) }
}
