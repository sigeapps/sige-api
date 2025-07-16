use crate::{tags::ISSUANCE_TAG, Result};
use application::{
    api::ApiContext,
    dtos::{
        parking::issuance::{returns::FinalizeIssuance, IssuanceSummary, StartIssuance},
        CommonQueryFilterDTO,
    },
    services::parking::issuance::IssuanceService,
};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct IssuanceBody {
    issuances: Vec<IssuanceSummary>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateIssuanceResponse {
    issuance_id: i32,
}

#[utoipa::path(
    get,
    path = "",
    tag = ISSUANCE_TAG,
    responses(
        (status = 200, description = "Listado de cargas", body = IssuanceBody),
    ),
 params(
            CommonQueryFilterDTO,
        )
)]
pub async fn get_issuances(
    Query(query): Query<CommonQueryFilterDTO>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Response> {
    let issuances = IssuanceService::find(ctx, query).await?;

    Ok((StatusCode::OK, Json(IssuanceBody { issuances })).into_response())
}

#[utoipa::path(
    post,
    path = "",
    tag = ISSUANCE_TAG,
    request_body = StartIssuance,
    responses(
        (status = 200, description = "El arma fue entregada", body = CreateIssuanceResponse)
    )
)]
pub async fn start_issuance(
    Extension(ctx): Extension<ApiContext>,
    Json(issuance): Json<StartIssuance>,
) -> Result<Response> {
    let issuance_id = IssuanceService::start(ctx, issuance).await?;

    Ok((StatusCode::OK, Json(CreateIssuanceResponse { issuance_id })).into_response())
}

#[utoipa::path(
    post,
    path = "/finalize",
    tag = ISSUANCE_TAG,
    request_body = FinalizeIssuance,
    responses(
        (status = 200, description = "La arma fue recuperada", body = CreateIssuanceResponse)
    )
)]
pub async fn finalize_issuance(
    Extension(ctx): Extension<ApiContext>,
    Json(issuance): Json<FinalizeIssuance>,
) -> Result<Response> {
    let issuance_id = IssuanceService::finalize(ctx, issuance).await?;

    Ok((StatusCode::OK, Json(CreateIssuanceResponse { issuance_id })).into_response())
}
