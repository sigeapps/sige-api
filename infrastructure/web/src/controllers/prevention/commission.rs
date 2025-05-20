use application::dtos::prevention::commission::CreateCommissionAggregateDto;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::repositories::commission_repository::CommissionAggregateRepository;

use crate::state::AppState;
use crate::Result;

pub async fn create_commission(
    State(app_state): State<AppState>,
    Json(commission): Json<CreateCommissionAggregateDto>,
) -> Result<Response> {
    let commission_id = app_state
        .commission_repository
        .create_with_relations(commission.into())
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(format!("commission_id: {}", commission_id)),
    )
        .into_response())
}
