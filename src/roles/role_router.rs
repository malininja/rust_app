use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/", get(super::role_handler::get_all))
}
