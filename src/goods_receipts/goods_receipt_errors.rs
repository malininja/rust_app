use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoodsReceiptError {
    #[error("Error getting goods receipts from the database")]
    GetAllError,

    #[error("Error getting goods receipt from the database")]
    GetByIdError,

    #[error("Goods receipt with provided ID doesn't exist")]
    NotFoundError,

    #[error("Error inserting goods receipt in the database")]
    CreateError,

    #[error("Error confirming goods receipt in the database")]
    ConfirmError,

    #[error("Goods receipt is already confirmed")]
    AlreadyConfirmedError,

    #[error("Error updating goods receipt in the database")]
    UpdateError,

    #[error("Error deleting goods receipt from the database")]
    DeleteError,
}
