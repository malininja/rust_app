use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvoiceError {
    #[error("Error getting invoices from the database")]
    GetAllError,

    #[error("Error getting invoice from the database")]
    GetByIdError,

    #[error("Invoice with provided ID doesn't exist")]
    NotFoundError,

    #[error("Error inserting invoice in the database")]
    CreateError,

    #[error("Error confirming invoice in the database")]
    ConfirmError,

    #[error("Invoice is already confirmed")]
    AlreadyConfirmedError,

    #[error("Error updating invoice in the database")]
    UpdateError,

    #[error("Error deleting invoice from the database")]
    DeleteError,
}
