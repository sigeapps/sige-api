use application::dtos::prevention::transport::CreateTransportDTO;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct GetTransportsQuery {
    search: Option<String>,
}

use application::api::ApiContext;
use application::services::prevention::transport::TransportService;
use axum::Extension;

pub async fn get_tranports(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<GetTransportsQuery>,
) -> Result<Response> {
    let transports = TransportService::find(ctx, query.search).await?;

    Ok((StatusCode::OK, Json(transports)).into_response())
}

pub async fn create_transport(
    Extension(ctx): Extension<ApiContext>,
    Json(transport): Json<CreateTransportDTO>,
) -> Result<Response> {
    TransportService::create(ctx, transport).await?;

    Ok((StatusCode::OK, "Transporte creado exitosamente").into_response())
}
