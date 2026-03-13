use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::users::user_model::UserModel;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub role_id: Uuid,
    pub username: String,
}

impl From<UserModel> for UserResponseDto {
    fn from(value: UserModel) -> Self {
        UserResponseDto {
            id: value.id,
            role_id: value.role_id,
            username: value.username,
        }
    }
}
