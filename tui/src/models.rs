use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Batch {
    pub batch_id: String,
    pub vendor_id: Option<String>,
    pub batch_size: Option<i32>,
    pub date_of_production: Option<NaiveDate>,
    pub qc_status: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub last_inspection_date: Option<NaiveDate>,
    pub fitment_date: Option<NaiveDate>,
    pub fitment_location: Option<String>,
    pub qr_hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Report {
    pub reportId: i32,
    pub batchId: String,
    pub inspectorName: String,
    pub remark: Option<String>,
    pub status: Option<i32>,
    pub createdAt: Option<DateTime<Utc>>, // string in API; chrono with serde feature handles RFC3339
}
