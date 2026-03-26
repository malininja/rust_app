use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::warehouse_stocks::warehouse_stock_model::WarehouseStockModel;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WarehouseStockResponseDto {
    pub id: Uuid,
    pub article_id: Uuid,
    pub quantity: Decimal,
}

impl From<WarehouseStockModel> for WarehouseStockResponseDto {
    fn from(model: WarehouseStockModel) -> Self {
        let WarehouseStockModel {
            id,
            article_id,
            quantity,
            ..
        } = model;

        Self {
            id,
            article_id,
            quantity,
        }
    }
}
