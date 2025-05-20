use application::dtos::prevention::transport::CreateTransportDTO;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use domain::entities::transport;
use domain::repositories::transport_repository::TransportRepository;
use serde::Deserialize;

use crate::state::AppState;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct GetTransportsQuery {
    search: Option<String>,
}

pub async fn get_tranports(
    State(app_state): State<AppState>,
    Query(query): Query<GetTransportsQuery>,
) -> Result<Response> {
    let transports = app_state.transport_repository.find(query.search).await?;

    Ok((StatusCode::OK, Json(transports)).into_response())
}

pub async fn create_transport(
    State(app_state): State<AppState>,
    Json(transport): Json<CreateTransportDTO>,
) -> Result<Response> {
    app_state
        .transport_repository
        .create(transport.into())
        .await?;

    Ok((StatusCode::OK, "Transport created successfully").into_response())
}
