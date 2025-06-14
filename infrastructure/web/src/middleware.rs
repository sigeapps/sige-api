use crate::{auth::AuthUser, Result};
use axum::{extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response};

use crate::error::WebError;

pub async fn auth_required(request: Request, next: Next) -> Result<Response> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(WebError::Unauthorized)?
        .to_str()
        .map_err(|_| WebError::Unauthorized)?;

    // Esperamos formato: "Bearer <token>" o solo "<token>"
    let token = if auth_header.starts_with("Bearer ") {
        auth_header.strip_prefix("Bearer ").unwrap()
    } else {
        auth_header
    };

    if token.is_empty() {
        return Err(WebError::Unauthorized);
    }

    if let Err(_) = AuthUser::from_jwt(token.to_string()) {
        return Err(WebError::Unauthorized);
    };

    Ok(next.run(request).await)
}
