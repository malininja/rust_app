use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::roles::{role_repository::PgRoleRepository, role_service};

pub async fn get_all(State(pool): State<PgPool>) -> Result<impl IntoResponse, StatusCode> {
    let repository = PgRoleRepository::new(pool);

    match role_service::get_all_roles(repository).await {
        Ok(res) => Ok(Json(res)),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
