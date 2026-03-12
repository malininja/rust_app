use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

use uuid::Uuid;

use crate::users::{
    dtos::{
        user_create_dto::UserCreateDto, user_response_dto::UserResponseDto,
        user_update_dto::UserUpdateDto,
    },
    user_errors::UserError,
    user_repository::UserRepository,
};

const LOG_CONTEXT: &str = "user_service";

pub async fn get_all_users<R: UserRepository>(
    repository: R,
) -> Result<Vec<UserResponseDto>, UserError> {
    match repository.get_all_users().await {
        Ok(user_models) => Ok(user_models.into_iter().map(|um| um.into()).collect()),
        Err(e) => {
            tracing::error!("{}: Get all users error: {}", LOG_CONTEXT, e);
            Err(UserError::GetUsersError)
        }
    }
}

pub async fn get_user_by_id<R: UserRepository>(
    repository: R,
    user_id: Uuid,
) -> Result<UserResponseDto, UserError> {
    match repository.get_user_by_id(user_id).await {
        Ok(Some(user_model)) => Ok(UserResponseDto::from(user_model)),
        Ok(None) => Err(UserError::UserNotFound),
        Err(e) => {
            tracing::error!(
                "{}: Get user by id error. id: {}. error: {}",
                LOG_CONTEXT,
                user_id,
                e
            );
            Err(UserError::GetUserByIdError)
        }
    }
}

pub async fn create_user<R: UserRepository>(
    repository: R,
    user: UserCreateDto,
) -> Result<UserResponseDto, UserError> {
    let hashed_password = hash_password(user.password).map_err(|e| {
        tracing::error!("{}: Hash password error: {}", LOG_CONTEXT, e);
        UserError::PasswordHashError
    })?;

    match repository
        .create_user(user.role_id, user.username, hashed_password)
        .await
    {
        Ok(user_model) => Ok(user_model.into()),
        Err(e) => {
            tracing::error!("{}: Create user error: {}", LOG_CONTEXT, e);
            Err(UserError::CreateUserError)
        }
    }
}

pub async fn update_user<R: UserRepository>(
    repository: R,
    id: Uuid,
    user: UserUpdateDto,
) -> Result<UserResponseDto, UserError> {
    let hashed_password = user.password
        .map(|p| hash_password(p))
        .transpose()
        .map_err(|e| {
            tracing::error!("{}: Hash password error: {}", LOG_CONTEXT, e);
            UserError::PasswordHashError
        })?;

    match repository
        .update_user(id, user.role_id, user.username, hashed_password)
        .await
    {
        Ok(Some(user_model)) => Ok(UserResponseDto::from(user_model)),
        Ok(None) => Err(UserError::UserNotFound),
        Err(e) => {
            tracing::error!("{}: Update user error: {}", LOG_CONTEXT, e);
            Err(UserError::UpdateUserError)
        }
    }
}

pub async fn soft_delete_user<R: UserRepository>(repository: R, id: Uuid) -> Result<(), UserError> {
    match repository.soft_delete_user(id).await {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(UserError::UserNotFound),
        Err(e) => {
            tracing::error!("{}: Soft delete user error: {}", LOG_CONTEXT, e);
            Err(UserError::SoftDeleteError)
        }
    }
}

pub async fn soft_undelete_user<R: UserRepository>(
    repository: R,
    id: Uuid,
) -> Result<(), UserError> {
    match repository.soft_undelete_user(id).await {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(UserError::UserNotFound),
        Err(e) => {
            tracing::error!("{}: Soft undelete user error: {}", LOG_CONTEXT, e);
            Err(UserError::SoftUndeleteError)
        }
    }
}

fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    Ok(argon.hash_password(password.as_bytes(), &salt)?.to_string())
}
