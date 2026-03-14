pub mod auth;
pub mod roles;
pub mod users;
use sqlx::postgres::PgPool;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn create_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/roles", roles::role_router::router())
        .nest("/users", users::user_router::router())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> String {
    "Hello world!".to_string()
}
