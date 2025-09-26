use crate::config::Config;
use crate::models::Batch;
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
}
