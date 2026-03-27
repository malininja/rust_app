use uuid::Uuid;

use crate::goods_receipts::{
    dtos::{
        goods_receipt_head_create_dto::GoodsReceiptHeadCreateDto,
        goods_receipt_head_response_dto::GoodsReceiptHeadResponseDto,
        goods_receipt_head_update_dto::GoodsReceiptHeadUpdateDto,
    },
    goods_receipt_errors::GoodsReceiptError,
    goods_receipt_repository::PgGoodsReceiptRepository,
};

const LOG_CONTEXT: &str = "goods_receipt_service";

pub async fn get_all_unconfirmed(
    repo: PgGoodsReceiptRepository,
) -> Result<Vec<GoodsReceiptHeadResponseDto>, GoodsReceiptError> {
    match repo.get_all_unconfirmed().await {
        Ok(items) => Ok(items
            .into_iter()
            .map(|i| GoodsReceiptHeadResponseDto::new(i, None))
            .collect()),
        Err(e) => {
            tracing::error!("{}: get_all_unconfirmed error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::GetAllError)
        }
    }
}

pub async fn get_by_id(
    repo: PgGoodsReceiptRepository,
    id: Uuid,
) -> Result<GoodsReceiptHeadResponseDto, GoodsReceiptError> {
    match repo.get_by_id(id).await {
        Ok(Some(item)) => Ok(GoodsReceiptHeadResponseDto::new(item.0, Some(item.1))),
        Ok(None) => Err(GoodsReceiptError::NotFoundError),
        Err(e) => {
            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::GetByIdError)
        }
    }
}

pub async fn create(
    repo: PgGoodsReceiptRepository,
    dto: GoodsReceiptHeadCreateDto,
) -> Result<GoodsReceiptHeadResponseDto, GoodsReceiptError> {
    match repo.create(dto.supplier_name, dto.items).await {
        Ok(item) => Ok(GoodsReceiptHeadResponseDto::new(item.0, Some(item.1))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::CreateError)
        }
    }
}

pub async fn confirm(repo: PgGoodsReceiptRepository, id: Uuid) -> Result<(), GoodsReceiptError> {
    match repo.get_by_id(id).await {
        Ok(Some(receipt)) => {
            if receipt.0.confirmed {
                return Err(GoodsReceiptError::AlreadyConfirmedError);
            }
        }
        Ok(None) => return Err(GoodsReceiptError::NotFoundError),
        Err(e) => {
            tracing::error!("{}: confirm error: {}", LOG_CONTEXT, e);
            return Err(GoodsReceiptError::ConfirmError);
        }
    }

    match repo.confirm(id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("{}: confirm error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::ConfirmError)
        }
    }
}

pub async fn update(
    repo: PgGoodsReceiptRepository,
    id: Uuid,
    dto: GoodsReceiptHeadUpdateDto,
) -> Result<GoodsReceiptHeadResponseDto, GoodsReceiptError> {
    let GoodsReceiptHeadUpdateDto {
        supplier_name,
        items,
    } = dto;

    match repo.update(id, supplier_name, items).await {
        Ok(item) => Ok(GoodsReceiptHeadResponseDto::new(item.0, Some(item.1))),
        Err(e) => {
            tracing::error!("{}: update error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::UpdateError)
        }
    }
}

pub async fn soft_delete(
    repo: PgGoodsReceiptRepository,
    id: Uuid,
) -> Result<(), GoodsReceiptError> {
    match repo.soft_delete(id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            tracing::error!("{}: soft delete error: {}", LOG_CONTEXT, e);
            Err(GoodsReceiptError::DeleteError)
        }
    }
}
