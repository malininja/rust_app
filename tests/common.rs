use rust_app::create_app;
use sqlx::PgPool;

pub struct TestApp {
    pub base_url: String,
    pub port: u16,
}

pub async fn create_test_app(pool: PgPool) -> TestApp {
    let base_url = "127.0.0.1".to_string();

    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind(format!("{base_url}:0"))
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(axum::serve(listener, app).into_future());

    TestApp { base_url, port }
}
