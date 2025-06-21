use crate::{auth::AuthClaims, Result};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use domain::auth::permissions::Permission;
use tracing::debug;

use crate::error::WebError;

pub async fn authenticate(mut request: Request, next: Next) -> Result<Response> {
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

    let claims = AuthClaims::from_jwt(token.to_string()).map_err(|_| WebError::Unauthorized)?;

    debug!("Authenticated user: {:?}", claims.user);

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn authorize(
    State(permission): State<Permission>,
    request: Request,
    next: Next,
) -> Result<Response> {
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

    let claims = AuthClaims::from_jwt(token.to_string()).map_err(|_| WebError::Unauthorized)?;

    println!("permissions: {:?}", claims.permissions);

    if !claims.permissions.contains(&permission) {
        return Err(WebError::Forbidden);
    };

    Ok(next.run(request).await)
}
