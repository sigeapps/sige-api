use crate::Result;
use application::dtos::personal::persona::CreatePersonaDTO;
use application::dtos::personal::persona::GetPersonaDTO;
use application::dtos::personal::persona::GetPersonaSummaryDTO;
use application::dtos::personal::persona::UpdatePersonaDTO;
use application::dtos::personal::persona::UpdatePersonaSummaryDTO;
use application::dtos::CommonQueryFilterDTO;
use application::dtos::PaginationDTO;
use axum::extract::Path;
use axum::extract::Query;
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

use application::api::ApiContext;
use application::services::personal::persona::PersonaService;
use axum::Extension;
use serde_json::json;

pub async fn create_persona(
    Extension(ctx): Extension<ApiContext>,
    Json(persona): Json<CreatePersonaDTO>,
) -> Result<Response> {
    let persona_id = PersonaService::create(ctx, persona).await?;

    Ok((
        StatusCode::CREATED,
        Json(PersonaBody::<i32> {
            persona: persona_id,
        }),
    )
        .into_response())
}

pub async fn update_persona(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(persona): Json<UpdatePersonaDTO>,
) -> Result<Response> {
    let persona_id = PersonaService::update(ctx, id, persona).await?;

    Ok((StatusCode::OK, Json(persona_id)).into_response())
}

pub async fn get_persona(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let persona = PersonaService::find_by_id(ctx, id).await?;

    Ok((
        StatusCode::OK,
        Json(PersonaBody::<GetPersonaDTO> { persona }),
    )
        .into_response())
}

pub async fn get_personas(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let personas: Vec<GetPersonaSummaryDTO> =
        PersonaService::find_summary(ctx.clone(), query.clone()).await?;
    let pagination = PersonaService::get_pagination(ctx, query).await?;

    Ok(Json(MultiplePersonasBody {
        personas,
        pagination,
    })
    .into_response())
}

pub async fn update_persona_summary(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(persona): Json<UpdatePersonaSummaryDTO>,
) -> Result<Response> {
    let id = PersonaService::update_summary(ctx, id, persona).await?;

    Ok(Json(json!({"persona_id": id})).into_response())
}
