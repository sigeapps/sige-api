use crate::Result;
use application::dtos::prevention::commission::{
    CreateCommissionAggregateDTO, GetCommissionSummaryDTO, UpdateCommissionExitDTO,
    UpdateCommissionStatusAggregateDTO,
};
use application::dtos::{CommonQueryFilterDTO, PaginationDTO};
use application::services::prevention::commission::CommissionService;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use application::api::ApiContext;
use axum::Extension;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct MultipleCommissionsBody {
    commissions: Vec<GetCommissionSummaryDTO>,
    pagination: PaginationDTO,
}

#[axum::debug_handler]
pub async fn create_commission(
    Extension(ctx): Extension<ApiContext>,
    Json(commission): Json<CreateCommissionAggregateDTO>,
) -> Result<Response> {
    CommissionService::create(ctx, commission).await?;
    Ok((StatusCode::CREATED, "Comisión creada exitosamente").into_response())
}

#[axum::debug_handler]
pub async fn get_commissions(
    Query(query): Query<CommonQueryFilterDTO>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Response> {
    let commissions = CommissionService::find(ctx.clone(), query.clone()).await?;
    let pagination = CommissionService::get_pagination(ctx, query).await?;

    Ok((
        StatusCode::OK,
        Json(MultipleCommissionsBody {
            commissions,
            pagination,
        }),
    )
        .into_response())
}

pub async fn get_commission_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    match CommissionService::find_by_id(ctx, id).await {
        Ok(commission) => Ok((StatusCode::OK, Json(commission)).into_response()),
        Err(e) => {
            tracing::error!("Error obteniendo comisión: {}", e.to_string());
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error obteniendo comisión",
            )
                .into_response())
        }
    }
}

#[axum::debug_handler]
pub async fn update_commission_exit(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCommissionExitDTO>,
) -> Result<Response> {
    CommissionService::update_exit(ctx, id, req).await?;
    Ok((StatusCode::OK, "Comisión actualizada exitosamente").into_response())
}

#[axum::debug_handler]
pub async fn update_commission_status(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCommissionStatusAggregateDTO>,
) -> Result<Response> {
    CommissionService::update_status(ctx, id, req).await?;
    Ok((
        StatusCode::OK,
        "Estado de la comisión actualizado exitosamente",
    )
        .into_response())
}

pub async fn get_commission_status_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let status = CommissionService::find_status_by_id(ctx, id).await?;

    Ok(Json(json!({"status": status})).into_response())
}
