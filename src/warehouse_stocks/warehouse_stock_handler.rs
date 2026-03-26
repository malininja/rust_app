use axum::{
    Json,
    extract::{Path, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    warehouse_stocks::{
        warehouse_stock_errors::WarehouseStockError,
        warehouse_stock_repository::PgWarehouseStockRepository, warehouse_stock_service,
    },
};

const LOG_CONTEXT: &str = "warehouse_stock_handler";

pub async fn get_all(State(app_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgWarehouseStockRepository::new(app_state.pool);
    match warehouse_stock_service::get_all(repo).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            tracing::error!("{}: get_all error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_article_id(
    State(app_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = PgWarehouseStockRepository::new(app_state.pool);

    match warehouse_stock_service::get_by_article_id(repo, article_id).await {
        Ok(item) => Ok(Json(item)),
        Err(WarehouseStockError::NotFoundError) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}: get_by_article_id error: {}", LOG_CONTEXT, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
