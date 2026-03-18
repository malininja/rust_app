use serde::{Deserialize, Serialize};

use crate::articles::article_model::UnitOfMeasure;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ArticleCreateDto {
    pub name: String,
    pub unit_of_measure: UnitOfMeasure,
}
