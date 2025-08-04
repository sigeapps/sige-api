use crate::{tags::INCLUSION_TAG, Result};
use application::{
    api::ApiContext, dtos::operations::inclusion::CreateInclusion,
    services::operations::inclusion::InclusionService,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateInclusionResponse {
    inclusion_id: i32,
}

#[utoipa::path(
    post,
    path = "",
    tag = INCLUSION_TAG,
    request_body = CreateInclusion,
    responses(
        (status = 201, description = "Inclusión creada", body = CreateInclusionResponse),
    )
)]
pub async fn create_inclusion(
    Extension(ctx): Extension<ApiContext>,
    Json(inclusion): Json<CreateInclusion>,
) -> Result<Response> {
    let inclusion_id = InclusionService::create_inclusion(ctx, inclusion).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateInclusionResponse { inclusion_id }),
    )
        .into_response())
}
