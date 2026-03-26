pub mod articles;
pub mod auth;
pub mod goods_receipts;
pub mod invoices;
pub mod roles;
pub mod users;
pub mod warehouse_stocks;

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
        .nest("/roles", roles::role_router::router(app_state.clone()))
        .nest("/users", users::user_router::router(app_state.clone()))
        .nest(
            "/articles",
            articles::article_router::router(app_state.clone()),
        )
        .nest(
            "/goods_receipts",
            goods_receipts::goods_receipt_router::router(app_state.clone()),
        )
        .nest(
            "/invoices",
            invoices::invoice_router::router(app_state.clone()),
        )
        .nest(
            "/warehouse_stocks",
            warehouse_stocks::warehouse_stock_router::router(app_state.clone()),
        )
        .route("/", get(handler))
        .nest("/login", auth::auth_router::router())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}

async fn handler() -> String {
    "Hello world!".to_string()
}
