use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, PartialEq)]
#[sqlx(type_name = "unit_of_measure", rename_all = "lowercase")]
pub enum UnitOfMeasure {
    Piece,
    Kg,
    Litre,
    Metre,
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
pub struct ArticleModel {
    pub id: Uuid,
    pub name: String,
    pub unit_of_measure: UnitOfMeasure,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
