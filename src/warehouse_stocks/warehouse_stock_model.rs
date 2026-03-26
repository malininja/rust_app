use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct WarehouseStockModel {
    pub id: Uuid,
    pub article_id: Uuid,
    pub quantity: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
