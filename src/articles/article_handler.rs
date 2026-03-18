use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    articles::{
        article_error::ArticleError,
        article_repository::PgArticleRepository,
        article_service::{
            create_article, get_all_articles, get_article_by_id, soft_delete_article,
            update_article,
        },
        dtos::{article_create_dto::ArticleCreateDto, article_update_dto::ArticleUpdateDto},
    },
};

const LOG_CONTEXT: &str = "article_handler";

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgArticleRepository::new(app_state.pool);

    match get_all_articles(repo).await {
        Ok(articles) => Ok(Json(articles)),
        Err(e) => {
            tracing::error!("{}: get_all_articles error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgArticleRepository::new(app_state.pool);

    match get_article_by_id(repo, id).await {
        Ok(article) => Ok(Json(article)),
        Err(e) => {
            if e == ArticleError::ArticleNotFoundError {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(article_create_dto): Json<ArticleCreateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgArticleRepository::new(app_state.pool);

    match create_article(repo, article_create_dto).await {
        Ok(article) => Ok((StatusCode::CREATED, Json(article))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(article_update_dto): Json<ArticleUpdateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgArticleRepository::new(app_state.pool);

    match update_article(repo, id, article_update_dto).await {
        Ok(article) => Ok(Json(article)),
        Err(e) => {
            if e == ArticleError::ArticleNotFoundError {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: update error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgArticleRepository::new(app_state.pool);

    match soft_delete_article(repo, id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(e) => {
            if e == ArticleError::ArticleNotFoundError {
                return Err(StatusCode::NOT_FOUND);
            }

            tracing::error!("{}: delete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
