use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{AppState, articles::article_handler, auth::auth_middlewares::logged_in};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(article_handler::get_all).post(article_handler::create),
        )
        .route(
            "/{id}",
            get(article_handler::get_by_id)
                .patch(article_handler::update)
                .delete(article_handler::delete),
        )
        .layer(from_fn_with_state(app_state, logged_in))
}
