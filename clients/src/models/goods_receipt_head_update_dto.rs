use serde::{Deserialize, Serialize};

use crate::models::goods_receipt_item_create_dto::GoodsReceiptItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsReceiptHeadUpdateDto {
    pub supplier_name: Option<String>,
    pub items: Option<Vec<GoodsReceiptItemCreateDto>>,
}
