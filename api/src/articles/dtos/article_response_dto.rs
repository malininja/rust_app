use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::articles::article_model::{ArticleModel, UnitOfMeasure};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ArticleResponseDto {
    pub id: Uuid,
    pub name: String,
    pub unit_of_measure: UnitOfMeasure,
}

impl From<ArticleModel> for ArticleResponseDto {
    fn from(article_model: ArticleModel) -> Self {
        let ArticleModel {
            id,
            name,
            unit_of_measure,
            ..
        } = article_model;

        Self {
            id,
            name,
            unit_of_measure,
        }
    }
}
