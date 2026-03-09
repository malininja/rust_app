use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct UserCreateDto {
    pub role_id: Uuid,
    pub username: String,
    pub password: String,
}
