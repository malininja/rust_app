use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}
