use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvoiceItemCreateDto {
    pub article_id: Uuid,
    pub ordinal: i32,
    pub quantity: Decimal,
}
