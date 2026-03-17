use serde::{Deserialize, Serialize};

use crate::articles::article_model::UnitOfMeasure;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ArticleUpdateDto {
    pub name: Option<String>,
    pub unit_of_measure: Option<UnitOfMeasure>,
}
