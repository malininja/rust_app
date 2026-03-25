use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::invoices::{
    dtos::invoice_item_response_dto::InvoiceItemResponseDto,
    invoice_head_model::InvoiceHeadModel,
    invoice_item_model::InvoiceItemModel,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadResponseDto {
    pub id: Uuid,
    pub customer_name: String,
    pub confirmed: bool,
    pub items: Option<Vec<InvoiceItemResponseDto>>,
}

impl InvoiceHeadResponseDto {
    pub fn new(
        head_model: InvoiceHeadModel,
        item_models: Option<Vec<InvoiceItemModel>>,
    ) -> Self {
        let InvoiceHeadModel {
            id,
            customer_name,
            confirmed,
            ..
        } = head_model;

        let items = item_models.map(|items| {
            let dtos = items
                .into_iter()
                .map(InvoiceItemResponseDto::from)
                .collect();
            dtos
        });

        Self {
            id,
            customer_name,
            confirmed,
            items,
        }
    }
}
