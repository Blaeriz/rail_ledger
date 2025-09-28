use crate::config::Config;
use crate::models::{Batch, Report, Vendor, User, Ticket, LiveMetricsMap};
use anyhow::{Context, Result};
use reqwest::Client;

pub struct Api {
    client: Client,
    cfg: Config,
}

impl Api {
    pub fn new(cfg: Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(cfg.request_timeout)
            .build()
            .context("building reqwest client")?;
        Ok(Self { client, cfg })
    }

    pub async fn batches(&self) -> Result<Vec<Batch>> {
        let url = format!("{}/api/batches", self.cfg.base_url);
        let resp = self.client.get(url).send().await.context("GET /api/batches")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/batches error {}: {}", code, body);
        }
        let items: Vec<Batch> = resp.json().await.context("parse /api/batches json")?;
        Ok(items)
    }

    pub async fn vendors(&self) -> Result<Vec<Vendor>> {
        let url = format!("{}/api/vendors", self.cfg.base_url);
        let resp = self.client.get(url).send().await.context("GET /api/vendors")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/vendors error {}: {}", code, body);
        }
        let items: Vec<Vendor> = resp.json().await.context("parse /api/vendors json")?;
        Ok(items)
    }

    pub async fn reports(&self) -> Result<Vec<Report>> {
        let url = format!("{}/api/reports", self.cfg.base_url);
        let resp = self.client.get(url).send().await.context("GET /api/reports")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/reports error {}: {}", code, body);
        }
        let items: Vec<Report> = resp.json().await.context("parse /api/reports json")?;
        Ok(items)
    }

    pub async fn users(&self) -> Result<Vec<User>> {
        let url = format!("{}/api/users", self.cfg.base_url);
        let resp = self.client.get(url).send().await.context("GET /api/users")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/users error {}: {}", code, body);
        }
        let items: Vec<User> = resp.json().await.context("parse /api/users json")?;
        Ok(items)
    }

    pub async fn tickets(&self) -> Result<Vec<Ticket>> {
        let url = format!("{}/api/tickets", self.cfg.base_url);
        let resp = self.client.get(url).send().await.context("GET /api/tickets")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/tickets error {}: {}", code, body);
        }
        let items: Vec<Ticket> = resp.json().await.context("parse /api/tickets json")?;
        Ok(items)
    }

    pub async fn batch_by_qr_hash(&self, qr_hash: &str) -> Result<Batch> {
        let url = format!("{}/api/batches/qr_hash", self.cfg.base_url);
        let resp = self
            .client
            .get(url)
            .query(&[("qr_hash", qr_hash)])
            .send()
            .await
            .context("GET /api/batches/qr_hash")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/batches/qr_hash error {}: {}", code, body);
        }
        let item: Batch = resp.json().await.context("parse /api/batches/qr_hash json")?;
        Ok(item)
    }

    pub async fn live_metrics(&self, minutes: u32) -> Result<LiveMetricsMap> {
        let url = format!("{}/api/metrics/live", self.cfg.base_url);
        let resp = self
            .client
            .get(url)
            .query(&[("minutes", minutes.to_string())])
            .send()
            .await
            .context("GET /api/metrics/live")?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("/api/metrics/live error {}: {}", code, body);
        }
        let map: LiveMetricsMap = resp.json().await.context("parse /api/metrics/live json")?;
        Ok(map)
    }
}
