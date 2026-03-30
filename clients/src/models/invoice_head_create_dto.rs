use serde::{Deserialize, Serialize};

use crate::models::invoice_item_create_dto::InvoiceItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadCreateDto {
    pub customer_name: String,
    pub items: Vec<InvoiceItemCreateDto>,
}
