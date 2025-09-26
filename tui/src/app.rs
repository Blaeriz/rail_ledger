use ratatui::widgets::TableState;

use crate::models;

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
    // Data
    pub batches: Vec<models::Batch>,
    pub batch_state: TableState,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab_index: 0,
            running: true,
            status: String::from("Press q to quit · Tab/Shift-Tab to switch tabs"),
            batches: Vec::new(),
            batch_state: TableState::default(),
        }
    }

    pub fn current_tab(&self) -> Tab {
        Tab::all()[self.tab_index]
    }
}
