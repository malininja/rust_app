pub mod roles;

use std::time::Duration;

use axum::{Router, extract::State, routing::get};
use sqlx::postgres::{PgPool, PgPoolOptions};

//#[tokio::main(flavor = "current_thread")] // single thread futures
//#[tokio::main(worker_threads = 4)] // explicitly set thread pool size
#[tokio::main] // number of logical cores on the machine
async fn main() {
    let _ = dotenvy::dotenv();
    let cnn_string = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cnn_string)
        .await
        .expect("Can't connect database.");

    let app = Router::new().route("/", get(handler))
    .route("/roles", get(roles::role_handler::get_all))
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(pool): State<PgPool>) -> String {
    sqlx::query_scalar::<_, String>("select 'Hello world!'")
        .fetch_one(&pool)
        .await
        .unwrap()
}
