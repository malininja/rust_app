use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDto {
    pub role_id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
}
