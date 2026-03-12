pub mod roles;
pub mod users;
use sqlx::postgres::PgPool;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/roles", roles::role_router::router())
        .nest("/users", users::user_router::router())
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> String {
    "Hello world!".to_string()
}
