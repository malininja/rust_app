use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    goods_receipts::{
        dtos::{
            goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
            goods_receipt_head_update_dto::GoodsReceiptHeadUpdateDto,
        },
        goods_receipt_errors::GoodsReceiptError,
        goods_receipt_repository::PgGoodsReceiptRepository,
        goods_receipt_service,
    },
};

const LOG_CONTEXT: &str = "goods_receipt_handler";

pub async fn get_all_unconfirmed(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgGoodsReceiptRepository::new(app_state.pool);
    match goods_receipt_service::get_all_unconfirmed(repo).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            tracing::error!("{}: get_all_unconfirmed error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgGoodsReceiptRepository::new(app_state.pool);

    match goods_receipt_service::get_by_id(repo, id).await {
        Ok(item) => Ok(Json(item)),
        Err(GoodsReceiptError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(dto): Json<GoodsReceiptHeadCreateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgGoodsReceiptRepository::new(app_state.pool);

    match goods_receipt_service::create(repo, dto).await {
        Ok(item) => Ok((StatusCode::CREATED, Json(item))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<GoodsReceiptHeadUpdateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgGoodsReceiptRepository::new(app_state.pool);

    match goods_receipt_service::update(repo, id, dto).await {
        Ok(item) => Ok(Json(item)),
        Err(GoodsReceiptError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: update error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgGoodsReceiptRepository::new(app_state.pool);

    match goods_receipt_service::soft_delete(repo, id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, ())),
        Err(GoodsReceiptError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: delete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
