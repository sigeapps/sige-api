use application::dtos::prevention::register::CreateRegister;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use domain::entities::register;
use domain::repositories::register_repository::RegisterRepository;
use sea_orm::ActiveValue::Set;
use tracing::error;

use crate::state::AppState;

pub async fn create_register(
    State(app_state): State<AppState>,
    Json(register): Json<CreateRegister>,
) -> impl IntoResponse {
    match app_state
        .register_repository
        .create(register::ActiveModel {
            last_name: Set(register.last_name),
            first_name: Set(register.first_name),
            is_official: Set(register.is_official.unwrap_or(false)),
            organism: Set(register.organism),
            visit_reason: Set(register.visit_reason),
            division: Set(register.division),
            observations: Set(register.observations),
            ci: Set(register.ci),
            photo: Set(register.photo),
            ..Default::default()
        })
        .await
    {
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

/// TODO: Make that this donts need all the Register model
/// TODO: Use the id path
#[axum::debug_handler]
pub async fn update_register_exit(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(register): Json<register::Model>,
) -> impl IntoResponse {
    match app_state.register_repository.update(register.into()).await {
        Ok(_) => (StatusCode::OK, "Register updated successfully").into_response(),

        Err(e) => {
            error!("Error updating register: {}", e.to_string());

            (StatusCode::INTERNAL_SERVER_ERROR, "Error updating register").into_response()
        }
    }
}
