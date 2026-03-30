use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceItemResponseDto {
    pub id: Uuid,
    pub invoice_head_id: Uuid,
    pub article_id: Uuid,
    pub ordinal: i32,
    pub quantity: Decimal,
}
