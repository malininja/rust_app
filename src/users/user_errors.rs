use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UserError {
    #[error("Error getting users from the database")]
    GetUsersError,

    #[error("Error getting user by id from the database")]
    GetUserByIdError,

    #[error("Error inserting user in the database")]
    CreateUserError,

    #[error("Error updating user in the database")]
    UpdateUserError,

    #[error("User with provided ID doesn't exist")]
    UserNotFound,

    #[error("Error creating password hash")]
    PasswordHashError,

    #[error("User already exists in the database")]
    UserAlreadyExists,

    #[error("Soft delete error")]
    SoftDeleteError,

    #[error("Soft undelete error")]
    SoftUndeleteError,
}
