use crate::auth::AuthClaims;
use crate::state::AppState;
use crate::Result;
use application::dtos::auth::{LoginRequest, RegisterRequest};
use application::dtos::user::CreateUserDTO;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody {
    token: String,
}

#[axum::debug_handler]
pub async fn login(
    State(app_state): State<AppState>,
    Json(form): Json<LoginRequest>,
) -> Result<Response> {
    debug!("Login attempt for user: {}", form.username);

    let user = app_state
        .user_service
        .find_by_username(form.username)
        .await?;

    if user.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid username or password"))
            .unwrap());
    }

    let user = user.unwrap();

    if verify_password(form.password, &user.password_hash).is_err() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid username or password"))
            .unwrap());
    }

    let permissions = app_state
        .user_service
        .find_permissions_by_role_id(user.role.id)
        .await?;

    let token = AuthClaims { user, permissions }.to_jwt()?;

    Ok((Json(UserBody { token })).into_response())
}

#[axum::debug_handler]
pub async fn register(
    State(app_state): State<AppState>,
    Json(form): Json<RegisterRequest>,
) -> Result<Response> {
    let user = CreateUserDTO::from(form);

    app_state.user_service.create(user).await?;

    debug!("User created successfully!");

    Ok(Redirect::to("/").into_response())
}

// TODO: MAKE LOGOUT AND REFRESH ROUTES
