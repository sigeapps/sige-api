use crate::auth::AuthSession;
use crate::state::AppState;
use crate::Result;
use application::dtos::auth::{LoginRequest, RegisterRequest};
use application::dtos::user::CreateUserDTO;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBody {
    session_id: Option<String>,
}

#[axum::debug_handler]
pub async fn login(
    session: Session,
    mut auth_session: AuthSession,
    Json(form): Json<LoginRequest>,
) -> Result<Response> {
    debug!("Login attempt for user: {}", form.username);

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

    debug!("User authenticated successfully");

    // After successful login, before cycling session:
    debug!("Session ID before cycling: {:?}", session.id());

    // Save session explicitly
    if let Err(e) = session.save().await {
        error!("Failed to save session: {}", e);
        return Ok((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session").into_response());
    }

    debug!("Session saved successfully");

    // Get the session ID before cycling (in case cycling fails)
    let current_session_id = session.id().map(|id| id.to_string());

    // Cycle session ID for security
    if let Err(e) = session.cycle_id().await {
        error!("Failed to cycle session ID: {}", e);
        // Don't fail the login, just log the error and continue with current session
        debug!("Continuing with current session ID due to cycle failure");
    }

    // Save the session again after cycling
    if let Err(e) = session.save().await {
        error!("Failed to save session after cycling: {}", e);
    }

    // Get the final session ID (after cycling if successful)
    let final_session_id = session.id().map(|id| id.to_string()).or(current_session_id);

    match form.next {
        Some(next) => Ok(Redirect::to(next.as_str()).into_response()),
        None => Ok(Json(LoginBody {
            session_id: final_session_id,
        })
        .into_response()),
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

#[axum::debug_handler]
pub async fn logout(
    session: Session,
    mut auth_session: AuthSession,
) -> Result<Response> {
    debug!("User logging out");

    // Logout from auth session
    if let Err(e) = auth_session.logout().await {
        error!("Failed to logout from auth session: {}", e);
    }

    // Clear the session
    if let Err(e) = session.delete().await {
        error!("Failed to delete session: {}", e);
        return Ok((StatusCode::INTERNAL_SERVER_ERROR, "Failed to clear session").into_response());
    }

    debug!("User logged out successfully");

    Ok((StatusCode::OK, "Logged out successfully").into_response())
}
