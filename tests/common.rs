use rust_app::{
    AppState,
    auth::dtos::{login_request_dto::LoginRequestDto, login_response_dto::LoginResponseDto},
    create_app,
};
use sqlx::PgPool;

pub const TEST_JWT_SECRET: &str = "long_random_string";
pub const ADMIN_USERNAME: &str = "admin";
pub const ADMIN_PASSWORD: &str = "123456";

pub struct TestApp {
    pub base_url: String,
    pub port: u16,
}

pub async fn create_test_app(state: AppState) -> TestApp {
    insert_admin_user(&state.pool).await;

    let base_url = "127.0.0.1".to_string();

    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind(format!("{base_url}:0"))
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(axum::serve(listener, app).into_future());

    TestApp { base_url, port }
}

async fn insert_admin_user(pool: &PgPool) {
    let _ = sqlx::query!("
        INSERT INTO users 
        (role_id, username, password) 
        VALUES (1, 'admin', '$argon2id$v=19$m=19456,t=2,p=1$dKfeZ4YkSzZg8+8ee7aB/w$grQunmZPhwcYgd50jG3LTYE8TNVc1oJBLDp6VUS18xU') 
        ON CONFLICT DO NOTHING
    ").execute(pool).await;
}

#[allow(dead_code)] // not use in some tests
pub async fn get_admin_token(app: &TestApp) -> String {
    let TestApp { base_url, port } = app;

    let reqwest_client = reqwest::Client::new();

    let login_dto = LoginRequestDto {
        username: ADMIN_USERNAME.to_string(),
        password: ADMIN_PASSWORD.to_string(),
    };

    let auth_res = reqwest_client
        .post(format!("http://{base_url}:{port}/login"))
        .json(&login_dto)
        .send()
        .await
        .unwrap()
        .json::<LoginResponseDto>()
        .await
        .unwrap();

    auth_res.token
}
