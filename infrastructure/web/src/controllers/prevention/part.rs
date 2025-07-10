use application::dtos::prevention::part::{
    CreatePartAggregateDTO, GetPartAggregateDTO, GetPartSummaryDTO, UpdatePartCompleteDTO,
};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::types::CommonQuery;
use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct PartBody<T> {
    part: T,
}

#[derive(Serialize, Deserialize)]
pub struct MultiplePartsBody {
    parts: Vec<GetPartSummaryDTO>,
}

use application::api::ApiContext;
use application::services::prevention::part::PartService;
use axum::Extension;

pub async fn create_part(
    Extension(ctx): Extension<ApiContext>,
    Json(part): Json<CreatePartAggregateDTO>,
) -> Result<Response> {
    PartService::create(ctx, part).await?;

    Ok((StatusCode::CREATED, Json("Parte creado exitosamente")).into_response())
}

pub async fn get_parts(
    Extension(ctx): Extension<ApiContext>,
    Query(_query): Query<CommonQuery>,
) -> Result<Response> {
    let parts = PartService::find(ctx).await?;

    Ok(Json(MultiplePartsBody { parts }).into_response())
}

pub async fn get_part_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let part = PartService::find_by_id(ctx, id).await?;

    Ok(Json(PartBody::<GetPartAggregateDTO> { part }).into_response())
}

#[axum::debug_handler]
pub async fn update_part_complete(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePartCompleteDTO>,
) -> Result<Response> {
    PartService::edit_part(ctx, id, req).await?;

    Ok(Json("Parte actualizado exitosamente").into_response())
}
