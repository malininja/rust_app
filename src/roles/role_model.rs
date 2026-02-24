use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct RoleModel {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}
