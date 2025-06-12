use crate::state::AppState;
use crate::Result;
use application::dtos::personal::persona::CreatePersonaDTO;
use application::dtos::personal::persona::GetPersonaSummaryDTO;
use application::dtos::CommonQueryFilterDTO;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PersonaBody<T> {
    persona: T,
}

pub async fn create_persona(
    State(app_state): State<AppState>,
    Json(persona): Json<CreatePersonaDTO>,
) -> Result<Response> {
    let persona_id = app_state.persona_service.create(persona).await?;

    Ok((
        StatusCode::CREATED,
        Json(PersonaBody::<i32> {
            persona: persona_id,
        }),
    )
        .into_response())
}

pub async fn get_personas(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let personas: Vec<GetPersonaSummaryDTO> = app_state.persona_service.find_summary(query).await?;

    Ok((StatusCode::OK, Json(personas)).into_response())
}
