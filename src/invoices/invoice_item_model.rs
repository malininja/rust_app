use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct InvoiceItemModel {
    pub id: Uuid,
    pub invoice_head_id: Uuid,
    pub article_id: Uuid,
    pub ordinal: i32,
    pub quantity: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
