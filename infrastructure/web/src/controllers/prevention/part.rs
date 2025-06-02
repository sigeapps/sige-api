use application::dtos::prevention::part::{
    CreatePartAggregateDTO, GetPartAggregateDTO, GetPartSummaryDTO, UpdatePartCompleteDTO,
};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::state::AppState;
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

pub async fn create_part(
    State(app_state): State<AppState>,
    Json(part): Json<CreatePartAggregateDTO>,
) -> Result<Response> {
    app_state.part_service.create(part).await?;

    Ok((StatusCode::CREATED, Json("part created successfully")).into_response())
}

pub async fn get_parts(
    State(app_state): State<AppState>,
    Query(_query): Query<CommonQuery>,
) -> Result<Response> {
    let parts = app_state.part_service.get_part_summaries().await?;

    Ok(Json(MultiplePartsBody { parts }).into_response())
}

pub async fn get_part_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let part = app_state.part_service.find_by_id(id).await?;

    Ok(Json(PartBody::<GetPartAggregateDTO> { part }).into_response())
}

#[axum::debug_handler]
pub async fn update_part_complete(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePartCompleteDTO>,
) -> Result<Response> {
    app_state.part_service.edit_part(id, req).await?;

    Ok(Json("part updated successfully").into_response())
}
