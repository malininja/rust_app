use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch},
};

use crate::{AppState, auth::auth_middlewares::auth_admin, users::user_handler};

pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(user_handler::get_all).post(user_handler::create))
        .route(
            "/{id}",
            get(user_handler::get_by_id)
                .patch(user_handler::update)
                .delete(user_handler::delete),
        )
        .route("/{id}/undelete", patch(user_handler::undelete))
        .layer(from_fn_with_state(app_state, auth_admin))
}
