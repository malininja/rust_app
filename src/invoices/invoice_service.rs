use uuid::Uuid;

use crate::invoices::{
    dtos::{
        invoice_head_create_dto::InvoiceHeadCreateDto,
        invoice_head_response_dto::InvoiceHeadResponseDto,
        invoice_head_update_dto::InvoiceHeadUpdateDto,
    },
    invoice_errors::InvoiceError,
    invoice_repository::PgInvoiceRepository,
};

const LOG_CONTEXT: &str = "invoice_service";

pub async fn get_all_unconfirmed(
    repo: PgInvoiceRepository,
) -> Result<Vec<InvoiceHeadResponseDto>, InvoiceError> {
    match repo.get_all_unconfirmed().await {
        Ok(items) => Ok(items
            .into_iter()
            .map(|i| InvoiceHeadResponseDto::new(i, None))
            .collect()),
        Err(e) => {
            tracing::error!("{}: get_all_unconfirmed error: {}", LOG_CONTEXT, e);
            Err(InvoiceError::GetAllError)
        }
    }
}

pub async fn get_by_id(
    repo: PgInvoiceRepository,
    id: Uuid,
) -> Result<InvoiceHeadResponseDto, InvoiceError> {
    match repo.get_by_id(id).await {
        Ok(Some(item)) => Ok(InvoiceHeadResponseDto::new(item.0, Some(item.1))),
        Ok(None) => Err(InvoiceError::NotFoundError),
        Err(e) => {
            tracing::error!("{}: get_by_id error: {}", LOG_CONTEXT, e);
            Err(InvoiceError::GetByIdError)
        }
    }
}

pub async fn create(
    repo: PgInvoiceRepository,
    dto: InvoiceHeadCreateDto,
) -> Result<InvoiceHeadResponseDto, InvoiceError> {
    match repo.create(dto.customer_name, dto.items).await {
        Ok(item) => Ok(InvoiceHeadResponseDto::new(item.0, Some(item.1))),
        Err(e) => {
            tracing::error!("{}: create error: {}", LOG_CONTEXT, e);
            Err(InvoiceError::CreateError)
        }
    }
}

pub async fn update(
    repo: PgInvoiceRepository,
    id: Uuid,
    dto: InvoiceHeadUpdateDto,
) -> Result<InvoiceHeadResponseDto, InvoiceError> {
    let InvoiceHeadUpdateDto {
        customer_name,
        confirmed,
        items,
    } = dto;

    match repo.update(id, customer_name, confirmed, items).await {
        Ok(Some(item)) => Ok(InvoiceHeadResponseDto::new(item.0, Some(item.1))),
        Ok(None) => Err(InvoiceError::NotFoundError),
        Err(e) => {
            tracing::error!("{}: update error: {}", LOG_CONTEXT, e);
            Err(InvoiceError::UpdateError)
        }
    }
}

pub async fn soft_delete(repo: PgInvoiceRepository, id: Uuid) -> Result<(), InvoiceError> {
    match repo.soft_delete(id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            tracing::error!("{}: soft delete error: {}", LOG_CONTEXT, e);
            Err(InvoiceError::DeleteError)
        }
    }
}
