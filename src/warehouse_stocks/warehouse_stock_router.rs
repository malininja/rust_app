use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{
    AppState, auth::auth_middlewares::logged_in, warehouse_stocks::warehouse_stock_handler,
};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(warehouse_stock_handler::get_all))
        .route("/{id}", get(warehouse_stock_handler::get_by_article_id))
        .layer(from_fn_with_state(app_state, logged_in))
}
