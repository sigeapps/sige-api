use crate::state::AppState;
use crate::Result;
use application::dtos::prevention::official::CreateOfficialDTO;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse, Json};
use domain::repositories::official_repository::OfficialRepository;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct GetOfficialsQuery {
    search: Option<String>,
    brigade_id: Option<i32>,
}

pub async fn get_officials(
    State(app_state): State<AppState>,
    Query(query): Query<GetOfficialsQuery>,
) -> Result<Response> {
    let officials = app_state
        .official_repository
        .find(query.search, query.brigade_id)
        .await?;

    Ok((StatusCode::OK, Json(officials)).into_response())
}

pub async fn get_official_by_id(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Response> {
    let official = app_state.official_repository.find_by_id(id).await?;

    Ok((StatusCode::OK, Json(official)).into_response())
}

pub async fn create_official(
    State(app_state): State<AppState>,
    Json(official): Json<CreateOfficialDTO>,
) -> Result<Response> {
    app_state
        .official_repository
        .create(official.into())
        .await?;

    Ok((StatusCode::OK, "Official created succesfully").into_response())
}
