use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Json;
use database::repositories::user_repository_impl::SeaOrmUserRepository;
use domain::entities::user;
use domain::repositories::user_repository::UserRepository;
use password_auth::generate_hash;
use sea_orm::ActiveValue::Set;
use tracing::{debug, error};

use crate::auth::AuthSession;
use crate::state::AppState;
use application::dtos::auth::{LoginRequest, RegisterRequest};

#[axum::debug_handler]
pub async fn login(
    mut auth_session: AuthSession<SeaOrmUserRepository>,
    Json(form): Json<LoginRequest>,
) -> impl IntoResponse {
    debug!("Someone trying to login!");

    let user = match auth_session.authenticate(form.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => {
            error!("{}", e.to_string());

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ERROR AUTHENTICATING USER",
            )
                .into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to login").into_response();
    }

    debug!("User authenticated succesfully");

    match form.next {
        Some(next) => Redirect::to(next.as_str()).into_response(),
        None => Redirect::to("/").into_response(),
    }
}

#[axum::debug_handler]
pub async fn register(
    State(app_state): State<AppState>,
    Json(form): Json<RegisterRequest>,
) -> impl IntoResponse {
    debug!("Registering user: {}", form.username);

    if let Err(e) = app_state.user_service.create(user).await {
        error!("Failed to create user: {}", e);

        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    debug!("User created successfully!");

    Redirect::to("/").into_response()
}
