use thiserror::Error;

#[derive(Error, Debug)]
pub enum WarehouseStockError {
    #[error("Error getting warehouse stocks from the database")]
    GetAllError,

    #[error("Error getting warehouse stock from the database")]
    GetByIdError,

    #[error("warehouse stock with provided ID doesn't exist")]
    NotFoundError,
}
