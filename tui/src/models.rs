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
    #[serde(rename = "reportId")]
    pub report_id: i32,
    #[serde(rename = "batchId")]
    pub batch_id: String,
    #[serde(rename = "inspectorName")]
    pub inspector_name: String,
    pub remark: Option<String>,
    pub status: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Vendor {
    pub vendor_id: String,
    pub no_of_batches: Option<i32>,
    pub gst_no: Option<String>,
    pub pan_number: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub audit_date: Option<NaiveDate>,
}
