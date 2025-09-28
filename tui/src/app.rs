use ratatui::widgets::TableState;

use crate::models;
use std::collections::HashMap;

pub const PAGE_SIZE: usize = 50;

#[derive(Copy, Clone, Debug)]
pub enum Tab {
    Overview,
    Batches,
    Vendors,
    Reports,
    Users,
    Tickets,
    AI,
    QR,
    System,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        use Tab::*;
        &[Overview, Batches, Vendors, Reports, Users, Tickets, AI, QR, System]
    }

    pub fn title(self) -> &'static str {
        use Tab::*;
        match self {
            Overview => "Overview",
            Batches => "Batches",
            Vendors => "Vendors",
            Reports => "Reports",
            Users => "Users",
            Tickets => "Tickets",
            AI => "AI",
            QR => "QR",
            System => "System",
        }
    }
}

pub struct App {
    pub tab_index: usize,
    pub running: bool,
    pub status: String,
    pub last_refresh: std::time::Instant,
    // Data
    pub batches: Vec<models::Batch>,
    pub batch_state: TableState,
    pub batch_page: usize,
    pub vendors: Vec<models::Vendor>,
    pub vendor_state: TableState,
    pub vendor_page: usize,
    pub reports: Vec<models::Report>,
    pub report_state: TableState,
    pub report_page: usize,
    pub users: Vec<models::User>,
    pub user_state: TableState,
    pub user_page: usize,
    pub tickets: Vec<models::Ticket>,
    pub ticket_state: TableState,
    pub ticket_page: usize,
    // QR search
    pub qr_input: String,
    pub qr_input_focused: bool,
    pub qr_result: Option<models::Batch>,
    pub qr_error: Option<String>,

    // Live metrics
    pub metrics_scale: MetricsScale,
    pub live_metrics: models::LiveMetricsMap,
    pub last_metrics_refresh: std::time::Instant,
    // Metrics UI selection
    pub metrics_routes: Vec<String>,
    pub metrics_route_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab_index: 0,
            running: true,
            status: String::from("Press q to quit · Tab/Shift-Tab to switch tabs"),
            last_refresh: std::time::Instant::now(),
            batches: Vec::new(),
            batch_state: TableState::default(),
            batch_page: 0,
            vendors: Vec::new(),
            vendor_state: TableState::default(),
            vendor_page: 0,
            reports: Vec::new(),
            report_state: TableState::default(),
            report_page: 0,
            users: Vec::new(),
            user_state: TableState::default(),
            user_page: 0,
            tickets: Vec::new(),
            ticket_state: TableState::default(),
            ticket_page: 0,
            qr_input: String::new(),
            qr_input_focused: true,
            qr_result: None,
            qr_error: None,
            metrics_scale: MetricsScale::Minutes(5),
            live_metrics: HashMap::new(),
            last_metrics_refresh: std::time::Instant::now(),
            metrics_routes: Vec::new(),
            metrics_route_index: 0,
        }
    }

    pub fn current_tab(&self) -> Tab {
        Tab::all()[self.tab_index]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MetricsScale {
    Minutes(u32),
    Hours(u32),
    Days(u32),
}

impl MetricsScale {
    pub fn to_minutes(self) -> u32 {
        match self {
            MetricsScale::Minutes(m) => m,
            MetricsScale::Hours(h) => h.saturating_mul(60),
            MetricsScale::Days(d) => d.saturating_mul(1440),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            MetricsScale::Minutes(_) => "min",
            MetricsScale::Hours(_) => "hour",
            MetricsScale::Days(_) => "day",
        }
    }
}
