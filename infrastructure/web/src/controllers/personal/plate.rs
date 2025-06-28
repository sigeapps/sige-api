use crate::{state::AppState, Result};
use application::dtos::personal::plate::PlateRequestDTO;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreatePlateResponse {
    id: i32,
}

pub async fn create_plate(
    State(app_state): State<AppState>,
    Json(plate_request): Json<PlateRequestDTO>,
) -> Result<Response> {
    let plate_id = app_state.plate_service.create(plate_request).await?;

    Ok(Json(CreatePlateResponse { id: plate_id }).into_response())
}
