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

#[derive(strum_macros::Display)]
pub enum Role {
    Admin,
    Sales,
    Warehouse,
}

impl TryFrom<i32> for Role {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Role::Admin),
            2 => Ok(Role::Sales),
            3 => Ok(Role::Warehouse),
            _ => Err("Role doesn't exist"),
        }
    }
}
