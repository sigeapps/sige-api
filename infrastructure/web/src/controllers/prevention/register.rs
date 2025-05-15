use crate::state::AppState;
use crate::Result;
use application::dtos::prevention::register::{CreateRegister, RegisterExit};
use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use domain::entities::register;
use domain::repositories::register_repository::RegisterRepository;
use domain::usecases::prevention::register::{RegisterExitInput, RegisterExitUseCase};
use sea_orm::ActiveValue::Set;
use tracing::error;

pub async fn create_register(
    State(app_state): State<AppState>,
    Json(register): Json<CreateRegister>,
) -> Result<Response> {
    app_state
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
        .await?;

    Ok((StatusCode::CREATED, "Register created successfully").into_response())
}

pub async fn get_registers(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state.register_repository.find().await {
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

/// TODO: Refactor this to an use case
#[axum::debug_handler]
pub async fn update_register_exit(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(register): Json<RegisterExitInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let register_exit_use_case = RegisterExitUseCase::new(app_state.register_repository);

    register_exit_use_case
        .execute(register, id)
        .await
        .map_err(|e| {
            error!("Error updating register exit use case: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}
