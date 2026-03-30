use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::unit_of_measure::UnitOfMeasure;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleResponseDto {
    pub id: Uuid,
    pub name: String,
    pub unit_of_measure: UnitOfMeasure,
}
