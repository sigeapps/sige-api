use crate::Result;
use application::api::ApiContext;
use application::dtos::personal::country::AddVerificationDTO;
use application::services::personal::country::CountryService;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountryVerificationBody<T> {
    pub verification: T,
}

pub async fn add_verification(
    Extension(ctx): Extension<ApiContext>,
    Json(dto): Json<AddVerificationDTO>,
) -> Result<Response> {
    let verification_id = CountryService::add_verification(ctx, dto).await?;

    Ok((
        StatusCode::CREATED,
        Json(CountryVerificationBody::<i32> {
            verification: verification_id,
        }),
    )
        .into_response())
}
