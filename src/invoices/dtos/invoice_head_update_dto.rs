use serde::{Deserialize, Serialize};

use crate::invoices::dtos::invoice_item_create_dto::InvoiceItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadUpdateDto {
    pub customer_name: Option<String>,
    pub confirmed: Option<bool>,
    pub items: Option<Vec<InvoiceItemCreateDto>>,
}
