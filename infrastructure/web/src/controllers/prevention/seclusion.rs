use application::dtos::prevention::seclusion::GetSeclusionDTO;
use application::dtos::prevention::seclusion::{
    visit::AddSeclusionVisitDTO, CreateSeclusionDTO, UpdateSeclusionExitDTO,
};
use application::dtos::{CommonQueryFilterDTO, PaginationDTO};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

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

// TODO: This is a temporary solution, we should use a proper DTO for this
//
use application::api::ApiContext;
use application::services::prevention::seclusion::SeclusionService;
use axum::Extension;

pub async fn create_seclusion(
    Extension(ctx): Extension<ApiContext>,
    Json(seclusion): Json<CreateSeclusionDTO>,
) -> Result<Response> {
    let seclusion_id = SeclusionService::create(ctx, seclusion).await?;

    Ok((
        StatusCode::CREATED,
        Json(SeclusionBody::<i32> {
            seclusion: seclusion_id,
        }),
    )
        .into_response())
}

pub async fn get_seclusions(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let seclusions = SeclusionService::find(ctx.clone(), query.clone()).await?;

    let pagination = SeclusionService::get_pagination(ctx, query.clone()).await?;

    Ok(Json(MultipleSeclusionsBody {
        seclusions,
        pagination,
    })
    .into_response())
}

pub async fn get_seclusion_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    match SeclusionService::find_by_id(ctx, id).await {
        Ok(Some(seclusion)) => {
            Ok((StatusCode::OK, Json(SeclusionBody { seclusion })).into_response())
        }

        Ok(None) => Ok((StatusCode::NOT_FOUND, "Reclusión no encontrada").into_response()),

        Err(e) => {
            error!("Error obteniendo reclusión: {}", e.to_string());

            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error obteniendo reclusión",
            )
                .into_response())
        }
    }
}

#[axum::debug_handler]
pub async fn update_seclusion_exit(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(seclusion): Json<UpdateSeclusionExitDTO>,
) -> Result<Response> {
    SeclusionService::update_exit(ctx, id, seclusion).await?;

    Ok((
        StatusCode::OK,
        "Salida de reclusión actualizada exitosamente",
    )
        .into_response())
}

pub async fn add_seclusion_visit(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(visit): Json<AddSeclusionVisitDTO>,
) -> Result<Response> {
    let visit_id = SeclusionService::add_visit(ctx, id, visit).await?;

    Ok((
        StatusCode::CREATED,
        Json(SeclusionBody::<i32> {
            seclusion: visit_id,
        }),
    )
        .into_response())
}
