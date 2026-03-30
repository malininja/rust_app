use serde::{Deserialize, Serialize};

use crate::models::unit_of_measure::UnitOfMeasure;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleUpdateDto {
    pub name: Option<String>,
    pub unit_of_measure: Option<UnitOfMeasure>,
}
