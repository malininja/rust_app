use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct GoodsReceiptHeadModel {
    pub id: Uuid,
    pub supplier_name: String,
    pub confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
