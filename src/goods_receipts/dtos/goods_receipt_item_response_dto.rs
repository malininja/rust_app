use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::goods_receipts::goods_receipt_item_model::GoodsReceiptItemModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsReceiptItemResponseDto {
    pub id: Uuid,
    pub goods_receipt_head_id: Uuid,
    pub article_id: Uuid,
    pub ordinal: i32,
    pub quantity: Decimal,
}

impl From<GoodsReceiptItemModel> for GoodsReceiptItemResponseDto {
    fn from(model: GoodsReceiptItemModel) -> Self {
        let GoodsReceiptItemModel {
            id,
            goods_receipt_head_id,
            article_id,
            ordinal,
            quantity,
            ..
        } = model;

        Self {
            id,
            goods_receipt_head_id,
            article_id,
            ordinal,
            quantity,
        }
    }
}
