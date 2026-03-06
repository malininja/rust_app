use rust_app::create_app;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, create_app(pool)).await.unwrap();
}
