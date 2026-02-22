use axum::{routing::get, Router};

//#[tokio::main(flavor = "current_thread")] // single thread futures
//#[tokio::main(worker_threads = 4)] // explicitly set thread pool size
#[tokio::main] // number of logical cores on the machine
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello world"
}