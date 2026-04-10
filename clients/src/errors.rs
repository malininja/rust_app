use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid status")]
    InvalidStatus,

    #[error("Result parse error")]
    ResultParseError,

    #[error("General error")]
    General,
}
