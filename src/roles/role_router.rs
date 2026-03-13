use axum::{Router, routing::get};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(super::role_handler::get_all))
}
