use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCreateDto {
    pub role_id: Uuid,
    pub username: String,
    pub password: String,
}
