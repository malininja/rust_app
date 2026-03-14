use axum::{
    extract::{FromRequestParts, Request, State},
    http::status::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{AppState, auth::claims::Claims, roles::role_model::Role};

pub async fn auth_admin(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let (mut parts, body) = request.into_parts();

    let claims = match Claims::from_request_parts(&mut parts, &state).await {
        Ok(claims) => claims,
        Err(status_code) => return status_code.into_response(),
    };

    if claims.role != Role::Admin.to_string() {
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(Request::from_parts(parts, body)).await
}
