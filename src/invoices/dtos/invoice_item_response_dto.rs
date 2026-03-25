use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::invoices::invoice_item_model::InvoiceItemModel;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InvoiceItemResponseDto {
    pub id: Uuid,
    pub invoice_head_id: Uuid,
    pub article_id: Uuid,
    pub ordinal: i32,
    pub quantity: Decimal,
}

impl From<InvoiceItemModel> for InvoiceItemResponseDto {
    fn from(model: InvoiceItemModel) -> Self {
        let InvoiceItemModel {
            id,
            invoice_head_id,
            article_id,
            ordinal,
            quantity,
            ..
        } = model;

        Self {
            id,
            invoice_head_id,
            article_id,
            ordinal,
            quantity,
        }
    }
}
