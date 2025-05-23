use application::dtos::prevention::commission::CreateCommissionAggregateDTO;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::state::AppState;
use crate::types::CommonQuery;
use crate::Result;

pub async fn create_commission(
    State(app_state): State<AppState>,
    Json(commission): Json<CreateCommissionAggregateDTO>,
) -> Result<Response> {
    let commission_id = app_state.commission_service.create(commission).await?;

    Ok((
        StatusCode::CREATED,
        Json(format!("commission_id: {}", commission_id)),
    )
        .into_response())
}

pub async fn get_commissions(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQuery>
) -> Result<Response> {
   let commissions = app_state.commission_service.find(query.search).await?;

   Ok(Json(commissions).into_response())
}
