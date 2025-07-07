use crate::{auth::JwtTrait, Result};
use application::{api::ApiContext, auth::UserClaims};
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use domain::auth::permissions::Permission;
use tracing::debug;

use crate::error::WebError;

pub async fn authenticate(mut request: Request, next: Next) -> Result<Response> {
    debug!("🔐 Iniciando proceso de autenticación");

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| {
            debug!("❌ Header Authorization no encontrado");
            WebError::Unauthorized
        })?
        .to_str()
        .map_err(|e| {
            debug!("❌ Error al convertir header Authorization: {:?}", e);
            WebError::Unauthorized
        })?;

    debug!("📝 Header Authorization recibido: {}", auth_header);

    // Esperamos formato: "Bearer <token>" o solo "<token>"
    let token = if auth_header.starts_with("Bearer ") {
        let extracted_token = auth_header.strip_prefix("Bearer ").unwrap();
        debug!(
            "🎫 Token extraído con prefijo Bearer: {}",
            &extracted_token[..50.min(extracted_token.len())]
        );
        extracted_token
    } else {
        debug!(
            "🎫 Token extraído sin prefijo: {}",
            &auth_header[..50.min(auth_header.len())]
        );
        auth_header
    };

    if token.is_empty() {
        debug!("❌ Token está vacío");
        return Err(WebError::Unauthorized);
    }

    debug!("🔍 Validando JWT token...");
    let claims = UserClaims::from_jwt(token.to_string()).map_err(|e| {
        debug!("❌ Error al validar JWT: {:?}", e);
        WebError::Unauthorized
    })?;

    debug!("✅ Authenticated user: {:?}", claims.user);

    // TODO: revisar si esto es eficiente en memoria, estamos clonando y volviendo a insertar la pool de la db

    if let Some(ext) = request.extensions_mut().get_mut::<ApiContext>() {
        ext.claims = Some(claims);
        let ext_cloned = ext.clone();

        debug!("extension {:?}", ext_cloned);

        request.extensions_mut().insert(ext_cloned);
    }

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

    let claims = UserClaims::from_jwt(token.to_string()).map_err(|_| WebError::Unauthorized)?;

    println!("permissions: {:?}", claims.permissions);

    if !claims.permissions.contains(&permission) {
        return Err(WebError::Forbidden);
    };

    Ok(next.run(request).await)
}

// Agregar handler para OPTIONS
pub async fn handle_preflight(request: Request, next: Next) -> Result<Response> {
    if request.method() == Method::OPTIONS {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
            )
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Max-Age", "86400")
            .body(axum::body::Body::empty())
            .unwrap());
    }

    Ok(next.run(request).await)
}
