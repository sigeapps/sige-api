use crate::state::AppState;
use crate::Result;
use application::dtos::prevention::register::CreateRegisterDTO;
use application::services::prevention::register::{RegisterExitInput, RegisterExitUseCase};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use chrono::{NaiveDate, NaiveDateTime};
use domain::repositories::register_repository::RegisterRepository;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Deserialize;
use tracing::error;

pub async fn create_register(
    State(app_state): State<AppState>,
    Json(register): Json<CreateRegisterDTO>,
) -> Result<Response> {
    app_state.register_service.create(register.into()).await?;

    Ok((StatusCode::CREATED, "Register created successfully").into_response())
}

#[derive(Debug, Deserialize)]
pub struct GetRegistersQuery {
    search: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    limit: Option<i32>,
    offset: Option<i32>,
}

pub async fn get_registers(
    State(app_state): State<AppState>,
    Query(query): Query<GetRegistersQuery>,
) -> Result<Response> {
    match app_state
        .register_service
        .find(
            query.search,
            query.from_date,
            query.to_date,
            query.limit,
            query.offset,
        )
        .await
    {
        Ok(registers) => Ok((StatusCode::OK, Json(registers)).into_response()),
        Err(e) => {
            error!("Error fetching registers: {}", e.to_string());
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching registers",
            )
                .into_response())
        }
    }
}

pub async fn get_register_by_id(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Response> {
    match app_state.register_service.find_by_id(id).await {
        Ok(Some(register)) => Ok((StatusCode::OK, Json(register)).into_response()),

        Ok(None) => Ok((StatusCode::NOT_FOUND, "Register not found").into_response()),

        Err(e) => {
            error!("Error fetching register: {}", e.to_string());

            Ok((StatusCode::INTERNAL_SERVER_ERROR, "Error fetching register").into_response())
        }
    }
}

/// TODO: Refactor this to an use case
#[axum::debug_handler]
pub async fn update_register_exit(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(register): Json<RegisterExitInput>,
) -> Result<Response> {
    let register_exit_use_case = RegisterExitUseCase::new(app_state.register_service);

    register_exit_use_case.execute(register, id).await?;

    Ok((StatusCode::OK, "Register updated successfully").into_response())
}
