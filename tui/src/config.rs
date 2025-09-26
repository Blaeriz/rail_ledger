use std::time::Duration;

pub struct Config {
    pub base_url: String,
    pub refresh_interval: Duration,
    pub request_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        let base_url = std::env::var("RAIL_LEDGER_API_BASE")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());

        let refresh_ms = std::env::var("REFRESH_INTERVAL_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30_000);

        let timeout_ms = std::env::var("REQUEST_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(8_000);

        Self {
            base_url,
            refresh_interval: Duration::from_millis(refresh_ms),
            request_timeout: Duration::from_millis(timeout_ms),
        }
    }
}
