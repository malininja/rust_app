use axum::{
    Router,
    routing::{get, patch},
};
use sqlx::PgPool;

use crate::users::user_handler::{self, create, get_all, get_by_id, undelete, update};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_all).post(create))
        .route(
            "/{id}",
            get(get_by_id).patch(update).delete(user_handler::delete),
        )
        .route("/{id}/undelete", patch(undelete))
}
