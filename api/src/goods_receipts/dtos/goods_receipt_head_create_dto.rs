use serde::{Deserialize, Serialize};

use crate::goods_receipts::dtos::goods_receipt_item_create_dto::GoodsReceiptItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsReceiptHeadCreateDto {
    pub supplier_name: String,
    pub items: Vec<GoodsReceiptItemCreateDto>,
}
