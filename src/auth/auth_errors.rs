use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Token creation error")]
    TokenCreationError,

    #[error("Database error")]
    DatabaseError,

    #[error("Password parse error")]
    PasswordParseError,

    #[error("Role map error")]
    RoleMapError,
}
