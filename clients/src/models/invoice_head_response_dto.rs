use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::invoice_item_response_dto::InvoiceItemResponseDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceHeadResponseDto {
    pub id: Uuid,
    pub customer_name: String,
    pub confirmed: bool,
    pub items: Option<Vec<InvoiceItemResponseDto>>,
}
