use uuid::Uuid;

use crate::warehouse_stocks::{
    warehouse_stock_errors::WarehouseStockError,
    warehouse_stock_repository::PgWarehouseStockRepository,
    warehouse_stock_response_dto::WarehouseStockResponseDto,
};

const LOG_CONTEXT: &str = "warehouse_stock_service";

pub async fn get_all(
    repo: PgWarehouseStockRepository,
) -> Result<Vec<WarehouseStockResponseDto>, WarehouseStockError> {
    match repo.get_all().await {
        Ok(items) => Ok(items
            .into_iter()
            .map(|i| WarehouseStockResponseDto::from(i))
            .collect()),
        Err(e) => {
            tracing::error!("{}: get_all_unconfirmed error: {}", LOG_CONTEXT, e);
            Err(WarehouseStockError::GetAllError)
        }
    }
}

pub async fn get_by_article_id(
    repo: PgWarehouseStockRepository,
    article_id: Uuid,
) -> Result<WarehouseStockResponseDto, WarehouseStockError> {
    match repo.get_by_article_id(article_id).await {
        Ok(Some(item)) => Ok(WarehouseStockResponseDto::from(item)),
        Ok(None) => Err(WarehouseStockError::NotFoundError),
        Err(e) => {
            tracing::error!("{}: get_by_article_id error: {}", LOG_CONTEXT, e);
            Err(WarehouseStockError::GetByIdError)
        }
    }
}
