use crate::tags::PERSONA_TAG;
use crate::Result;
use application::dtos::personal::persona::child::Child;
use application::dtos::personal::persona::course::Course;
use application::dtos::personal::persona::educational::Educational;
use application::dtos::personal::persona::health::Health;
use application::dtos::personal::persona::labor::Labor;
use application::dtos::personal::persona::operational::Operational;
use application::dtos::personal::persona::record::Record;
use application::dtos::personal::persona::relative::Relative;
use application::dtos::personal::persona::situation::UpdateSituationDTO;
use application::dtos::personal::persona::traits::Traits;
use application::dtos::personal::persona::CreatePersonaDTO;
use application::dtos::personal::persona::CreatePersonaSummaryDTO;
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

#[derive(Serialize, Deserialize, ToSchema)]
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
use utoipa::ToSchema;

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

#[utoipa::path(
    post,
    path = "",
    request_body = CreatePersonaSummaryDTO,
    responses(
        (status = 201, description = "Persona creada exitosamente", body = PersonaBody<i32>),
        (status = 400, description = "Solicitud inválida"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_persona_summary(
    Extension(ctx): Extension<ApiContext>,
    Json(persona): Json<CreatePersonaSummaryDTO>,
) -> Result<Response> {
    let persona_id = PersonaService::create_summary(&ctx, persona).await?;

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

macro_rules! update_entity {
    ($fn_name:ident, $dto_type:ty) => {
        pub async fn $fn_name(
            Extension(ctx): Extension<ApiContext>,
            Path(id): Path<i32>,
            Json(entity): Json<$dto_type>,
        ) -> Result<impl IntoResponse> {
            PersonaService::$fn_name(&ctx, id, entity).await?;

            Ok(StatusCode::OK.into_response())
        }
    };
}

update_entity!(update_traits, Traits);
update_entity!(update_health, Health);
update_entity!(update_situation, UpdateSituationDTO);

macro_rules! add_entitys {
    ($fn_name:ident, $dto_type:ty) => {
        pub async fn $fn_name(
            Extension(ctx): Extension<ApiContext>,
            Path(id): Path<i32>,
            Json(entity): Json<Vec<$dto_type>>,
        ) -> Result<impl IntoResponse> {
            PersonaService::$fn_name(&ctx, id, entity).await?;

            Ok(StatusCode::OK.into_response())
        }
    };
}

add_entitys!(add_childrens, Child);
add_entitys!(add_operational, Operational);
add_entitys!(add_relatives, Relative);
add_entitys!(add_courses, Course);
add_entitys!(add_education, Educational);
add_entitys!(add_work_experience, Labor);
add_entitys!(add_records, Record);
