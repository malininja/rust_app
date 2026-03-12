use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::users::{
    dtos::{user_create_dto::UserCreateDto, user_update_dto::UserUpdateDto},
    user_errors::UserError,
    user_repository::PgUserRepository,
    user_service,
};

const LOG_CONTEXT: &str = "user_handler";

pub async fn get_all(State(pool): State<PgPool>) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

    match user_service::get_all_users(repo).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}: get_all error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

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
    State(pool): State<PgPool>,
    Json(body): Json<UserCreateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

    match user_service::create_user(repo, body.role_id, body.username, body.password).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UserUpdateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

    match user_service::update_user(repo, id, body.role_id, body.username, body.password).await {
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
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

    match user_service::soft_delete_user(repo, id).await {
        Ok(_) => Ok(()),
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
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgUserRepository::new(pool);

    match user_service::soft_undelete_user(repo, id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            if e == UserError::UserNotFound {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: undelete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
