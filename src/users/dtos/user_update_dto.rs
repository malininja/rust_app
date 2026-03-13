use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateDto {
    pub role_id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}
