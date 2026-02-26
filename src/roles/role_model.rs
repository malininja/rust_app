use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct RoleModel {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
