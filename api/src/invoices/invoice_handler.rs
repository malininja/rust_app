use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    invoices::{
        dtos::{
            invoice_head_create_dto::InvoiceHeadCreateDto,
            invoice_head_update_dto::InvoiceHeadUpdateDto,
        },
        invoice_errors::InvoiceError,
        invoice_repository::PgInvoiceRepository,
        invoice_service,
    },
};

const LOG_CONTEXT: &str = "invoice_handler";

pub async fn get_all_unconfirmed(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgInvoiceRepository::new(app_state.pool);
    match invoice_service::get_all_unconfirmed(repo).await {
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
    let repo = PgInvoiceRepository::new(app_state.pool);

    match invoice_service::get_by_id(repo, id).await {
        Ok(item) => Ok(Json(item)),
        Err(InvoiceError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(dto): Json<InvoiceHeadCreateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgInvoiceRepository::new(app_state.pool);

    match invoice_service::create(repo, dto).await {
        Ok(item) => Ok((StatusCode::CREATED, Json(item))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn confirm(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgInvoiceRepository::new(app_state.pool);

    match invoice_service::confirm(repo, id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, ())),
        Err(InvoiceError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: confirm error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<InvoiceHeadUpdateDto>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgInvoiceRepository::new(app_state.pool);

    match invoice_service::update(repo, id, dto).await {
        Ok(item) => Ok(Json(item)),
        Err(InvoiceError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(InvoiceError::AlreadyConfirmedError) => Err(StatusCode::CONFLICT),
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
    let repo = PgInvoiceRepository::new(app_state.pool);

    match invoice_service::soft_delete(repo, id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, ())),
        Err(InvoiceError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: delete error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
