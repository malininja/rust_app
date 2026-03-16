use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}
