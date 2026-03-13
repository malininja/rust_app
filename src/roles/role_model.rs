use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Clone, PartialEq, Debug, Deserialize)]
pub struct RoleModel {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
