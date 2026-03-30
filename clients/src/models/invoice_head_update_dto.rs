use serde::{Deserialize, Serialize};

use crate::models::invoice_item_create_dto::InvoiceItemCreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadUpdateDto {
    pub customer_name: Option<String>,
    pub items: Option<Vec<InvoiceItemCreateDto>>,
}
