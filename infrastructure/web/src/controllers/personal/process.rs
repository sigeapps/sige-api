use crate::Result;
use application::{
    api::ApiContext,
    dtos::{personal::correspondence::CreateCorrespondenceRequest, CommonQueryFilterDTO},
    services::personal::correspondence::CorrespondenceService,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

#[axum::debug_handler]
pub async fn create_correspondence(
    Extension(ctx): Extension<ApiContext>,
    Json(dto): Json<CreateCorrespondenceRequest>,
) -> Result<impl IntoResponse> {
    let id = CorrespondenceService::create(ctx, dto).await?;

    Ok((StatusCode::CREATED, Json(json!({"correspondence_id": id}))))
}

#[axum::debug_handler]
pub async fn get_correspondences(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<impl IntoResponse> {
    let correspondences = CorrespondenceService::find(ctx.clone(), query.clone()).await?;
    let pagination = CorrespondenceService::get_pagination(ctx, query).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"pagination": pagination, "correspondences": correspondences})),
    ))
}

#[axum::debug_handler]
pub async fn get_correspondence(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    let correspondence = CorrespondenceService::find_by_id(ctx, id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"correspondence": correspondence})),
    ))
}
