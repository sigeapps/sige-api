use crate::Result;
use application::dtos::prevention::official::CreateOfficialDTO;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetOfficialsQuery {
    search: Option<String>,
    brigade_id: Option<i32>,
}

use application::api::ApiContext;
use application::services::prevention::official::OfficialService;
use axum::Extension;

pub async fn get_officials(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<GetOfficialsQuery>,
) -> Result<Response> {
    let officials = OfficialService::find(ctx, query.search, query.brigade_id).await?;

    Ok((StatusCode::OK, Json(officials)).into_response())
}

pub async fn get_official_by_id(
    Extension(ctx): Extension<ApiContext>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Response> {
    let official = OfficialService::find_by_id(ctx, id).await?;

    Ok((StatusCode::OK, Json(official)).into_response())
}

pub async fn create_official(
    Extension(ctx): Extension<ApiContext>,
    Json(official): Json<CreateOfficialDTO>,
) -> Result<Response> {
    OfficialService::create(ctx, official).await?;

    Ok((StatusCode::OK, "Oficial creado exitosamente").into_response())
}
