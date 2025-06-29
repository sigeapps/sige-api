use crate::{state::AppState, Result};
use application::dtos::personal::plate::{PlateRequestDTO, PlateResponseDTO};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreatePlateResponse {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct PlateBody {
    plate: Option<PlateResponseDTO>,
}

pub async fn create_plate(
    State(app_state): State<AppState>,
    Json(plate_request): Json<PlateRequestDTO>,
) -> Result<Response> {
    let plate_id = app_state.plate_service.create(plate_request).await?;

    Ok(Json(CreatePlateResponse { id: plate_id }).into_response())
}

// TODO: Regresar error si no se encuentra la plancha

pub async fn get_plate_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let plate = app_state.plate_service.find_by_id(id).await?;

    Ok(Json(PlateBody { plate }).into_response())
}
