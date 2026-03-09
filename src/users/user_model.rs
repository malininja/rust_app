use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, PartialEq, Debug)]
pub struct UserModel {
    pub id: Uuid,
    pub role_id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
