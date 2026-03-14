use axum::{Router, routing::post};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(super::auth_handler::login))
}
