pub mod roles;
use sqlx::postgres::PgPool;

use axum::{Router, extract::State, routing::get};

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/roles", roles::role_router::router())
        .with_state(pool)
}

async fn handler(State(pool): State<PgPool>) -> String {
    sqlx::query_scalar::<_, String>("select 'Hello world!'")
        .fetch_one(&pool)
        .await
        .unwrap()
}
