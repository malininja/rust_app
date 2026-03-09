use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct UserUpdateDto {
    pub role_id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
}
