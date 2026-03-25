use serde::{Deserialize, Serialize};

use crate::invoices::dtos::invoice_item_create_dto::InvoiceItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadCreateDto {
    pub supplier_name: String,
    pub items: Vec<InvoiceItemCreateDto>,
}
