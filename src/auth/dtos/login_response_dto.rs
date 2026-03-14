use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LoginResponseDto {
    pub token: String,
}
