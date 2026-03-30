use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub role_id: i32,
    pub username: String,
}
