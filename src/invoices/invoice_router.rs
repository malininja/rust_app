use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{AppState, auth::auth_middlewares::logged_in, invoices::invoice_handler};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(invoice_handler::get_all_unconfirmed).post(invoice_handler::create),
        )
        .route(
            "/{id}",
            get(invoice_handler::get_by_id)
                .patch(invoice_handler::update)
                .delete(invoice_handler::delete),
        )
        .layer(from_fn_with_state(app_state, logged_in))
}
