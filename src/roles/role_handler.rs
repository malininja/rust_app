use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{AppState, roles::{role_repository::PgRoleRepository, role_service}};

pub async fn get_all(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let repository = PgRoleRepository::new(state.pool);

    match role_service::get_all_roles(repository).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("role_handler: get_all error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
