use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::goods_receipt_item_response_dto::GoodsReceiptItemResponseDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsReceiptHeadResponseDto {
    pub id: Uuid,
    pub supplier_name: String,
    pub confirmed: bool,
    pub items: Option<Vec<GoodsReceiptItemResponseDto>>,
}

