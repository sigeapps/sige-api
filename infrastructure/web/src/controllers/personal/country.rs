use crate::state::AppState;
use crate::Result;
use application::dtos::personal::country::AddVerificationDTO;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountryVerificationBody<T> {
    pub verification: T,
}

pub async fn add_verification(
    State(app_state): State<AppState>,
    Json(dto): Json<AddVerificationDTO>,
) -> Result<Response> {
    let verification_id = app_state.country_service.add_verification(dto).await?;

    Ok((
        StatusCode::CREATED,
        Json(CountryVerificationBody::<i32> {
            verification: verification_id,
        }),
    )
        .into_response())
}
