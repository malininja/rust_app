use serde::{Deserialize, Serialize};

use crate::models::unit_of_measure::UnitOfMeasure;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleCreateDto {
    pub name: String,
    pub unit_of_measure: UnitOfMeasure,
}
