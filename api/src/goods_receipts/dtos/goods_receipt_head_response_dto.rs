use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::goods_receipts::{
    dtos::goods_receipt_item_response_dto::GoodsReceiptItemResponseDto,
    goods_receipt_head_model::GoodsReceiptHeadModel,
    goods_receipt_item_model::GoodsReceiptItemModel,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsReceiptHeadResponseDto {
    pub id: Uuid,
    pub supplier_name: String,
    pub confirmed: bool,
    pub items: Option<Vec<GoodsReceiptItemResponseDto>>,
}

impl GoodsReceiptHeadResponseDto {
    pub fn new(
        head_model: GoodsReceiptHeadModel,
        item_models: Option<Vec<GoodsReceiptItemModel>>,
    ) -> Self {
        let GoodsReceiptHeadModel {
            id,
            supplier_name,
            confirmed,
            ..
        } = head_model;

        let items = item_models.map(|items| {
            let dtos: Vec<GoodsReceiptItemResponseDto> = items
                .into_iter()
                .map(GoodsReceiptItemResponseDto::from)
                .collect();
            dtos
        });

        Self {
            id,
            supplier_name,
            confirmed,
            items,
        }
    }
}
