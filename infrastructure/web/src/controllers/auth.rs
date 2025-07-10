use crate::auth::JwtTrait;
use crate::Result;
use application::api::ApiContext;
use application::auth::UserClaims;
use application::dtos::auth::LoginRequest;
use application::services::user::UserService;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody {
    token: String,
}

#[axum::debug_handler]
pub async fn login(
    Extension(ctx): Extension<ApiContext>,
    Json(form): Json<LoginRequest>,
) -> Result<Response> {
    debug!(
        "Intento de inicio de sesión para el usuario: {}",
        form.username
    );

    let user = UserService::find_by_username(ctx.clone(), form.username).await?;

    if user.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Usuario o contraseña inválidos"))
            .unwrap());
    }

    let user = user.unwrap();

    if verify_password(form.password, &user.password_hash).is_err() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Usuario o contraseña inválidos"))
            .unwrap());
    }

    let permissions = UserService::find_permissions_by_role_id(ctx, user.role.id).await?;

    println!("{:?}", permissions);

    let token = UserClaims { user, permissions }.to_jwt()?;

    Ok((Json(UserBody { token })).into_response())
}

pub async fn get_current_user(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    Ok(Json(ctx.claims).into_response())
}
