use application::dtos::prevention::transport::{CreateTransportDTO, GetTransportDTO};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde::Deserialize;
use utoipa::IntoParams;

use crate::tags::TRANSPORT_TAG;
use crate::Result;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTransportsQuery {
    search: Option<String>,
}

use application::api::ApiContext;
use application::services::prevention::transport::TransportService;
use axum::Extension;

#[utoipa::path(get, path = "", tag = TRANSPORT_TAG,
    responses(
    (status = 200, description = "Lista de registros", body = [GetTransportDTO]),
),
 params(
            GetTransportsQuery,
        ),
)]
pub async fn get_tranports(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<GetTransportsQuery>,
) -> Result<Response> {
    let transports = TransportService::find(ctx, query.search).await?;

    Ok((StatusCode::OK, Json(transports)).into_response())
}

#[utoipa::path(post, path = "", tag = TRANSPORT_TAG,
    request_body = CreateTransportDTO,
    responses(
    (status = 201, description = "El transporte se creo de manera exitosa"),
)
)]
pub async fn create_transport(
    Extension(ctx): Extension<ApiContext>,
    Json(transport): Json<CreateTransportDTO>,
) -> Result<Response> {
    TransportService::create(ctx, transport).await?;

    Ok((StatusCode::OK, "Transporte creado exitosamente").into_response())
}
