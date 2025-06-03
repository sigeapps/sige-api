use application::dtos::prevention::commission::{
    CreateCommissionAggregateDTO, GetCommissionAggregateDTO, GetCommissionStatusAggregateDTO,
    GetCommissionSummaryDTO, UpdateCommissionExitDTO, UpdateCommissionStatusAggregateDTO,
};
use application::dtos::{CommonQueryFilterDTO, PaginationDTO};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct CommissionBody<T> {
    commission: T,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleCommissionsBody {
    commissions: Vec<GetCommissionSummaryDTO>,
    pagination: PaginationDTO,
}

pub async fn create_commission(
    State(app_state): State<AppState>,
    Json(commission): Json<CreateCommissionAggregateDTO>,
) -> Result<Response> {
    let commission_id = app_state.commission_service.create(commission).await?;

    Ok((
        StatusCode::CREATED,
        Json(CommissionBody::<i32> {
            commission: commission_id,
        }),
    )
        .into_response())
}

pub async fn get_commissions(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let commissions = app_state
        .commission_service
        .clone()
        .find(query.clone())
        .await?;
    let pagination = app_state
        .commission_service
        .clone()
        .get_pagination(query.clone())
        .await?;

    Ok(Json(MultipleCommissionsBody {
        commissions,
        pagination,
    })
    .into_response())
}

pub async fn get_commission_status(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let commission = app_state.commission_service.find_status_by_id(id).await?;

    Ok(Json(commission).into_response())
}

#[axum::debug_handler]
pub async fn update_commission_exit(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCommissionExitDTO>,
) -> Result<Response> {
    app_state.commission_service.update_exit(id, req).await?;

    Ok(Json("commission exit updated").into_response())
}

pub async fn update_commission_status(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCommissionStatusAggregateDTO>,
) -> Result<Response> {
    app_state.commission_service.update_status(id, req).await?;

    Ok(Json("commission status updated").into_response())
}

pub async fn get_commission_status_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let commission = app_state.commission_service.find_status_by_id(id).await?;

    Ok(Json(CommissionBody::<GetCommissionStatusAggregateDTO> { commission }).into_response())
}

pub async fn get_commission_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let commission = app_state.commission_service.find_by_id(id).await?;

    Ok(Json(CommissionBody::<GetCommissionAggregateDTO> { commission }).into_response())
}
