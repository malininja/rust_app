use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{AppState, auth::auth_middlewares::auth_admin};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(super::role_handler::get_all))
        .layer(from_fn_with_state(app_state, auth_admin))
}
