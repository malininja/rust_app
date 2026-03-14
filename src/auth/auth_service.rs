use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};

use crate::{
    auth::{auth_errors::AuthError, claims::Claims},
    roles::role_model::Role,
    users::user_repository::UserRepository,
};

const LOG_CONTEXT: &str = "auth_service";

pub async fn login<R: UserRepository>(
    user_repo: R,
    username: String,
    password: String,
    jwt_secret: String,
) -> Result<String, AuthError> {
    let user = user_repo
        .get_user_by_username(username)
        .await
        .map_err(|e| {
            tracing::error!("{}: login error fetching user from db: {}", LOG_CONTEXT, e);
            AuthError::DatabaseError
        })?
        .ok_or(AuthError::UserNotFound)?;

    let password_hash = PasswordHash::new(&user.password).map_err(|e| {
        tracing::error!("{}: login error parsing password: {}", LOG_CONTEXT, e);
        AuthError::PasswordParseError
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| AuthError::InvalidPassword)?;

    let role = Role::try_from(user.role_id).map_err(|e| {
        tracing::error!("{}: login error mapping role: {}", LOG_CONTEXT, e);
        AuthError::RoleMapError
    })?;

    let claims = Claims {
        sub: user.id.to_string(),
        role: role.to_string(),
        exp: Utc::now().timestamp() as usize + 3600,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("{}: login token creation error: {}", LOG_CONTEXT, e);
        AuthError::TokenCreationError
    })?;

    Ok(token)
}
