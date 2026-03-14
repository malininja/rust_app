use axum::{Json, extract::State, http::status::StatusCode, response::IntoResponse};

use crate::{
    AppState,
    auth::{
        auth_errors::AuthError,
        auth_service,
        dtos::{login_request_dto::LoginRequestDto, login_response_dto::LoginResponseDto},
    },
    users::user_repository::PgUserRepository,
};

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequestDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match auth_service::login(repo, body.username, body.password, state.jwt_secret).await {
        Ok(token) => Ok(Json(LoginResponseDto { token })),
        Err(e) => match e {
            AuthError::UserNotFound | AuthError::InvalidPassword => Err(StatusCode::UNAUTHORIZED),
            _ => {
                tracing::error!("auth_handler: login error: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}
