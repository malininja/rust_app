use axum::{
    Router,
    routing::{get, patch},
};

use crate::{AppState, users::user_handler::{self, create, get_all, get_by_id, undelete, update}};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all).post(create))
        .route(
            "/{id}",
            get(get_by_id).patch(update).delete(user_handler::delete),
        )
        .route("/{id}/undelete", patch(undelete))
}
