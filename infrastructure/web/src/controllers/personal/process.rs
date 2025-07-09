use crate::Result;
use application::{
    api::ApiContext, dtos::personal::correspondence::CreateCorrespondenceRequest,
    services::personal::correspondence::CorrespondenceService,
};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

#[axum::debug_handler]
pub async fn create_correspondence(
    Extension(ctx): Extension<ApiContext>,
    Json(dto): Json<CreateCorrespondenceRequest>,
) -> Result<impl IntoResponse> {
    let id = CorrespondenceService::create(ctx, dto).await?;

    Ok((StatusCode::CREATED, Json(json!({"correspondence_id": id}))))
}
