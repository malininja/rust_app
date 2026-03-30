use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct WarehouseStockResponseDto {
    pub id: Uuid,
    pub article_id: Uuid,
    pub quantity: Decimal,
}
