use crate::state::AppState;
use crate::Result;
use application::dtos::personal::persona::CreatePersonaDTO;
use application::dtos::personal::persona::GetPersonaDTO;
use application::dtos::personal::persona::GetPersonaSummaryDTO;
use application::dtos::personal::persona::UpdatePersonaDTO;
use application::dtos::CommonQueryFilterDTO;
use application::dtos::PaginationDTO;
use axum::extract::Path;
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

#[derive(Serialize, Deserialize)]
pub struct MultiplePersonasBody {
    pub personas: Vec<GetPersonaSummaryDTO>,
    pub pagination: PaginationDTO,
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

pub async fn update_persona(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(persona): Json<UpdatePersonaDTO>,
) -> Result<Response> {
    let persona_id = app_state.persona_service.update(id, persona).await?;

    Ok((StatusCode::OK, Json(persona_id)).into_response())
}

pub async fn get_persona(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let persona = app_state.persona_service.find_by_id(id).await?;

    Ok((
        StatusCode::OK,
        Json(PersonaBody::<GetPersonaDTO> { persona }),
    )
        .into_response())
}

pub async fn get_personas(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let personas: Vec<GetPersonaSummaryDTO> = app_state
        .persona_service
        .find_summary(query.clone())
        .await?;
    let pagination = app_state.persona_service.get_pagination(query).await?;

    Ok(Json(MultiplePersonasBody {
        personas,
        pagination,
    })
    .into_response())
}
