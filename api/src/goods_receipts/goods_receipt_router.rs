use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch},
};

use crate::{AppState, auth::auth_middlewares::logged_in, goods_receipts::goods_receipt_handler};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(goods_receipt_handler::get_all_unconfirmed).post(goods_receipt_handler::create),
        )
        .route(
            "/{id}",
            get(goods_receipt_handler::get_by_id)
                .patch(goods_receipt_handler::update)
                .delete(goods_receipt_handler::delete),
        )
        .route("/{id}/confirm", patch(goods_receipt_handler::confirm))
        .layer(from_fn_with_state(app_state, logged_in))
}
