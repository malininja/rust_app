use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    users::{
        dtos::{user_create_dto::UserCreateDto, user_update_dto::UserUpdateDto},
        user_errors::UserError,
        user_repository::PgUserRepository,
        user_service,
    },
};

const LOG_CONTEXT: &str = "user_handler";

pub async fn get_all(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::get_all_users(repo).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}: get_all error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::get_user_by_id(repo, id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            if e == UserError::UserNotFound {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<UserCreateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::create_user(repo, body).await {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UserUpdateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::update_user(repo, id, body).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            if e == UserError::UserNotFound {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: update error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::soft_delete_user(repo, id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(e) => {
            if e == UserError::UserNotFound {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: delete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn undelete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(state.pool);

    match user_service::soft_undelete_user(repo, id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(e) => {
            if e == UserError::UserNotFound {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: undelete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
