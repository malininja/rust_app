use axum::{extract::FromRequestParts, http::request::Parts, http::status::StatusCode};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(token) => Ok(token.claims),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
