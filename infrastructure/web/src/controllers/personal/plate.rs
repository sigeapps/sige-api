use crate::Result;
use application::dtos::personal::plate::{GetPlateDTO, PlateRequestDTO, PlateResponseDTO};
use application::dtos::CommonQueryFilterDTO;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreatePlateResponse {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct PlateBody {
    plate: Option<PlateResponseDTO>,
}

#[derive(Serialize, Deserialize)]
struct MultiplePlatesBody {
    plates: Vec<GetPlateDTO>,
}

use application::api::ApiContext;
use application::services::personal::plate::PlateService;
use axum::Extension;

pub async fn create_plate(
    Extension(ctx): Extension<ApiContext>,
    Json(plate_request): Json<PlateRequestDTO>,
) -> Result<Response> {
    let plate_id = PlateService::create(ctx, plate_request).await?;

    Ok(Json(CreatePlateResponse { id: plate_id }).into_response())
}

// TODO: Regresar error si no se encuentra la plancha

pub async fn get_plate_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let plate = PlateService::find_by_id(ctx, id).await?;

    Ok(Json(PlateBody { plate }).into_response())
}

pub async fn get_plates(
    Extension(ctx): Extension<ApiContext>,
    Query(filter): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let plates = PlateService::find(ctx, filter).await?;

    Ok(Json(MultiplePlatesBody { plates }).into_response())
}
