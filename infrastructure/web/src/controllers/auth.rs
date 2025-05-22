use application::dtos::user::CreateUserDTO;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use tracing::{debug, error};

use crate::auth::AuthSession;
use crate::state::AppState;
use crate::Result;
use application::dtos::auth::{LoginRequest, RegisterRequest};

#[axum::debug_handler]
pub async fn login(
    mut auth_session: AuthSession,
    Json(form): Json<LoginRequest>,
) -> Result<Response> {
    debug!("Someone trying to login!");

    let user = match auth_session.authenticate(form.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok((StatusCode::UNAUTHORIZED, "invalid credentials").into_response()),
        Err(e) => {
            error!("{}", e.to_string());

            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERROR AUTHENTICATING USER",
            )
                .into_response());
        }
    };

    if auth_session.login(&user).await.is_err() {
        return Ok((StatusCode::INTERNAL_SERVER_ERROR, "Failed to login").into_response());
    }

    debug!("User authenticated succesfully");

    match form.next {
        Some(next) => Ok(Redirect::to(next.as_str()).into_response()),
        None => Ok(Redirect::to("/").into_response()),
    }
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
