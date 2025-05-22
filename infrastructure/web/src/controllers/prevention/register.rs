use crate::state::AppState;
use crate::Result;
use application::dtos::prevention::register::{CreateRegisterDTO, UpdateRegisterExitDTO};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use axum::{extract::State, response::IntoResponse};
use chrono::NaiveDate;
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
    limit: Option<u64>,
    offset: Option<u64>,
}

pub async fn get_registers(
    State(app_state): State<AppState>,
    Query(query): Query<GetRegistersQuery>,
) -> Result<Response> {
    let registers = app_state
        .register_service
        .find(
            query.search,
            query.from_date,
            query.to_date,
            query.limit,
            query.offset,
        )
        .await?;

    Ok((StatusCode::OK, Json(registers)).into_response())
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
    Json(register): Json<UpdateRegisterExitDTO>,
) -> Result<Response> {
    app_state.register_service.update_exit(register, id).await?;

    Ok((StatusCode::OK, "register updated successfully").into_response())
}
