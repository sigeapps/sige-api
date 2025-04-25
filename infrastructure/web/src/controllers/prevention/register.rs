use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use domain::entities::register;
use domain::repositories::register_repository::RegisterRepository;
use tracing::error;

use crate::state::AppState;

pub async fn create_register(
    State(app_state): State<AppState>,
    Json(register): Json<register::Model>,
) -> impl IntoResponse {
    match app_state.register_repository.create(register.into()).await {
        Ok(_) => return (StatusCode::CREATED, "Register created successfully").into_response(),
        Err(e) => {
            error!("Error creating register: {}", e.to_string());

            return (StatusCode::INTERNAL_SERVER_ERROR, "Error creating register").into_response();
        }
    };
}

pub async fn get_registers(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state.register_repository.find_all().await {
        Ok(registers) => (StatusCode::OK, Json(registers)).into_response(),
        Err(e) => {
            error!("Error fetching registers: {}", e.to_string());
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching registers",
            )
                .into_response()
        }
    }
}

pub async fn get_register_by_id(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    match app_state.register_repository.find_by_id(id).await {
        Ok(Some(register)) => (StatusCode::OK, Json(register)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Register not found").into_response(),
        Err(e) => {
            error!("Error fetching register: {}", e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching register").into_response()
        }
    }
}
