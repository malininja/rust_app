use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateDto {
    pub role_id: i32,
    pub username: String,
    pub password: String,
}
