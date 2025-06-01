use application::dtos::prevention::seclusion::GetSeclusionDTO;
use application::dtos::prevention::seclusion::{
    visit::AddSeclusionVisitDTO, CreateSeclusionDTO, UpdateSeclusionExitDTO,
};
use application::dtos::{CommonQueryFilterDTO, PaginationDTO};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::state::AppState;
use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct SeclusionBody<T> {
    seclusion: T,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleSeclusionsBody {
    seclusions: Vec<GetSeclusionDTO>,
    pagination: PaginationDTO,
}

pub async fn create_seclusion(
    State(app_state): State<AppState>,
    Json(seclusion): Json<CreateSeclusionDTO>,
) -> Result<Response> {
    let seclusion_id = app_state.seclusion_service.create(seclusion).await?;

    Ok((
        StatusCode::CREATED,
        Json(SeclusionBody::<i32> {
            seclusion: seclusion_id,
        }),
    )
        .into_response())
}

pub async fn get_seclusions(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let seclusions = app_state
        .clone()
        .seclusion_service
        .find(query.clone())
        .await?;

    let pagination = app_state
        .clone()
        .seclusion_service
        .get_pagination(query.clone())
        .await?;

    Ok(Json(MultipleSeclusionsBody {
        seclusions,
        pagination,
    })
    .into_response())
}

pub async fn get_seclusion_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    match app_state.seclusion_service.find_by_id(id).await {
        Ok(Some(seclusion)) => {
            Ok((StatusCode::OK, Json(SeclusionBody { seclusion })).into_response())
        }

        Ok(None) => Ok((StatusCode::NOT_FOUND, "Seclusion not found").into_response()),

        Err(e) => {
            error!("Error fetching seclusion: {}", e.to_string());

            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching seclusion",
            )
                .into_response())
        }
    }
}

#[axum::debug_handler]
pub async fn update_seclusion_exit(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(seclusion): Json<UpdateSeclusionExitDTO>,
) -> Result<Response> {
    app_state
        .seclusion_service
        .update_exit(id, seclusion)
        .await?;

    Ok((StatusCode::OK, "Seclusion exit updated successfully").into_response())
}

pub async fn add_seclusion_visit(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(visit): Json<AddSeclusionVisitDTO>,
) -> Result<Response> {
    let visit_id = app_state.seclusion_service.add_visit(id, visit).await?;

    Ok((
        StatusCode::CREATED,
        Json(SeclusionBody::<i32> {
            seclusion: visit_id,
        }),
    )
        .into_response())
}
