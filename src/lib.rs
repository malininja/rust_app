pub mod articles;
pub mod auth;
pub mod roles;
pub mod users;
use sqlx::postgres::PgPool;

use axum::{Router, middleware::from_fn_with_state, routing::get};
use tower_http::trace::TraceLayer;

use crate::auth::auth_middlewares::auth_admin;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn create_app(app_state: AppState) -> Router {
    Router::new()
        .nest("/roles", roles::role_router::router())
        .nest("/users", users::user_router::router())
        .layer(from_fn_with_state(app_state.clone(), auth_admin))
        .route("/", get(handler))
        .nest("/login", auth::auth_router::router())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> String {
    "Hello world!".to_string()
}
