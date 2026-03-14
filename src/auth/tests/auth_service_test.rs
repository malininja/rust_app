use chrono::Utc;
use uuid::Uuid;

use crate::{
    auth::{auth_errors::AuthError, auth_service},
    users::{tests::user_mock_repository::UserMockRepo, user_model::UserModel},
};

const PASSWORD: &str = "123456";

fn create_user() -> UserModel {
    UserModel {
        id: Uuid::new_v4(),
        role_id: 1,
        username: "admin".to_string(),
        password: "$argon2id$v=19$m=19456,t=2,p=1$BrvXn7A75XO6Z5bnm/QzUw$8Dbxj/eUI5+tAN1RMYfCE/Fe2v2mF+DXiYgZ803D73o".to_string(), // password = 123456
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    }
}

#[tokio::test]
async fn login_user_database_error() {
    let mut repo = UserMockRepo::new();
    repo.get_by_username_result = Some(Err(()));

    let username = "username".to_string();

    let result = auth_service::login(
        &repo,
        username.clone(),
        "password".to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap_err();

    let captured_username = repo
        .captured_get_by_name_args
        .lock()
        .unwrap()
        .take()
        .unwrap();

    assert_eq!(username, captured_username);
    assert_eq!(AuthError::DatabaseError, result);
}

#[tokio::test]
async fn login_user_not_found() {
    let repo = UserMockRepo::new();

    let result = auth_service::login(
        &repo,
        "username".to_string(),
        "password".to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap_err();

    assert_eq!(AuthError::UserNotFound, result);
}

#[tokio::test]
async fn login_password_parse_error() {
    let mut user = create_user();
    user.password = "invalid password".to_string();

    let mut repo = UserMockRepo::new();
    repo.get_by_username_result = Some(Ok(user));

    let result = auth_service::login(
        &repo,
        "username".to_string(),
        "password".to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap_err();

    assert_eq!(AuthError::PasswordParseError, result);
}

#[tokio::test]
async fn login_invalid_password() {
    let user = create_user();

    let mut repo = UserMockRepo::new();
    repo.get_by_username_result = Some(Ok(user));

    let result = auth_service::login(
        &repo,
        "username".to_string(),
        "wrong password".to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap_err();

    assert_eq!(AuthError::InvalidPassword, result);
}

#[tokio::test]
async fn role_map_error() {
    let mut user = create_user();
    user.role_id = -123;

    let mut repo = UserMockRepo::new();
    repo.get_by_username_result = Some(Ok(user));

    let result = auth_service::login(
        &repo,
        "username".to_string(),
        PASSWORD.to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap_err();

    assert_eq!(AuthError::RoleMapError, result);
}

#[tokio::test]
async fn success() {
    let user = create_user();

    let mut repo = UserMockRepo::new();
    repo.get_by_username_result = Some(Ok(user));

    let result = auth_service::login(
        &repo,
        "username".to_string(),
        PASSWORD.to_string(),
        "jwt_secret".to_string(),
    )
    .await
    .unwrap();

    assert!(!result.is_empty());
}
