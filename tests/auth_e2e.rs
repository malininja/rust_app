mod common;

use reqwest::StatusCode;
use rust_app::{AppState, auth::dtos::login_request_dto::LoginRequestDto};
use sqlx::PgPool;

use crate::common::{ADMIN_PASSWORD, ADMIN_USERNAME, TEST_JWT_SECRET, create_test_app};

#[sqlx::test]
async fn auth_login_ok(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let reqwest_client = reqwest::Client::new();

    let login_dto = LoginRequestDto {
        username: ADMIN_USERNAME.to_string(),
        password: ADMIN_PASSWORD.to_string(),
    };

    let auth_res = reqwest_client
        .post(format!(
            "http://{}:{}/login",
            test_app.base_url, test_app.port
        ))
        .json(&login_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, auth_res.status());
}

#[sqlx::test]
async fn auth_login_unathorized_invalid_username(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let reqwest_client = reqwest::Client::new();

    let login_dto = LoginRequestDto {
        username: "wrong_username".to_string(),
        password: ADMIN_PASSWORD.to_string(),
    };

    let auth_res = reqwest_client
        .post(format!(
            "http://{}:{}/login",
            test_app.base_url, test_app.port
        ))
        .json(&login_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, auth_res.status());
}

#[sqlx::test]
async fn auth_login_unathorized_invalid_password(pool: PgPool) {
    let test_app = create_test_app(AppState {
        pool,
        jwt_secret: TEST_JWT_SECRET.to_string(),
    })
    .await;

    let reqwest_client = reqwest::Client::new();

    let login_dto = LoginRequestDto {
        username: ADMIN_USERNAME.to_string(),
        password: "wrong_password".to_string(),
    };

    let auth_res = reqwest_client
        .post(format!(
            "http://{}:{}/login",
            test_app.base_url, test_app.port
        ))
        .json(&login_dto)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, auth_res.status());
}
