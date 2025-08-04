use crate::tags::PERSONA_TAG;
use crate::Result;
use application::dtos::personal::persona::child::Child;
use application::dtos::personal::persona::conyuge::GetConyugeDTO;
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
use application::dtos::personal::persona::GetPersonaDTO;
use application::dtos::personal::persona::GetPersonaSummaryDTO;
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MultiplePersonasBody {
    pub personas: Vec<GetPersonaSummaryDTO>,
    pub pagination: PaginationDTO,
}

use application::api::ApiContext;
use application::services::personal::persona::PersonaService;
use axum::Extension;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "",
    request_body = CreatePersonaDTO,
    responses(
        (status = 201, description = "Persona creada exitosamente", body = PersonaBody<i32>),
        (status = 400, description = "Solicitud inválida"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_persona(
    Extension(ctx): Extension<ApiContext>,
    Json(persona): Json<CreatePersonaDTO>,
) -> Result<Response> {
    let persona_id = PersonaService::create_complete(ctx, persona).await?;

    Ok((
        StatusCode::CREATED,
        Json(PersonaBody::<i32> {
            persona: persona_id,
        }),
    )
        .into_response())
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "ID de la persona")
    ),
    responses(
        (status = 200, description = "Persona encontrada exitosamente", body = PersonaBody<GetPersonaDTO>),
        (status = 404, description = "Persona no encontrada"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
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

#[utoipa::path(
    get,
    path = "",
    params(
        CommonQueryFilterDTO
    ),
    responses(
        (status = 200, description = "Lista de personas encontrada exitosamente", body = MultiplePersonasBody),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
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

// ========================================
// CONTROLADORES PARA CREAR ELEMENTOS INDIVIDUALES
// ========================================

/// Macro para crear controladores de creación de elementos individuales
macro_rules! create_entity {
    ($fn_name:ident, $service_fn:ident, $dto_type:ty, $path:literal, $description:literal) => {
        #[utoipa::path(
                            post,
                            path = "",
                            request_body = $dto_type,
                            params(
                                ("id" = i32, Path, description = "ID de la persona")
                            ),
                            responses(
                                (status = 201, description = $description, body = PersonaBody<i32>),
                                (status = 400, description = "Solicitud inválida"),
                                (status = 404, description = "Persona no encontrada"),
                                (status = 500, description = "Error interno del servidor")
                            ),
                            tag = PERSONA_TAG
                        )]
        pub async fn $fn_name(
            Extension(ctx): Extension<ApiContext>,
            Path(persona_id): Path<i32>,
            Json(entity): Json<$dto_type>,
        ) -> Result<Response> {
            let entity_id = PersonaService::$service_fn(&ctx, persona_id, entity).await?;

            Ok((
                StatusCode::CREATED,
                Json(PersonaBody::<i32> { persona: entity_id }),
            )
                .into_response())
        }
    };
}

create_entity!(
    create_operational,
    create_operational,
    Operational,
    "/{id}/operational",
    "Información operacional creada exitosamente"
);

create_entity!(
    create_relative,
    create_relative,
    Relative,
    "/{id}/relatives",
    "Familiar creado exitosamente"
);

create_entity!(
    create_education,
    create_education,
    Educational,
    "/{id}/education",
    "Educación creada exitosamente"
);

create_entity!(
    create_course,
    create_course,
    Course,
    "/{id}/courses",
    "Curso creado exitosamente"
);

create_entity!(
    create_work_experience,
    create_work_experience,
    Labor,
    "/{id}/work-experience",
    "Experiencia laboral creada exitosamente"
);

create_entity!(
    create_child,
    create_child,
    Child,
    "/{id}/children",
    "Hijo creado exitosamente"
);

create_entity!(
    create_record,
    create_record,
    Record,
    "/{id}/records",
    "Antecedente creado exitosamente"
);

/// Controlador para crear cónyuge
#[utoipa::path(
    post,
    path = "",
    request_body = GetConyugeDTO,
    params(
        ("id" = i32, Path, description = "ID de la persona")
    ),
    responses(
        (status = 201, description = "Cónyuge creado exitosamente"),
        (status = 400, description = "Solicitud inválida"),
        (status = 404, description = "Persona no encontrada"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_spouse(
    Extension(ctx): Extension<ApiContext>,
    Path(persona_id): Path<i32>,
    Json(spouse): Json<GetConyugeDTO>,
) -> Result<Response> {
    PersonaService::upsert_spouse(&ctx, persona_id, spouse).await?;

    Ok(StatusCode::CREATED.into_response())
}

/// Controlador para crear características físicas
#[utoipa::path(
    post,
    path = "",
    request_body = Traits,
    params(
        ("id" = i32, Path, description = "ID de la persona")
    ),
    responses(
        (status = 201, description = "Características físicas creadas exitosamente"),
        (status = 400, description = "Solicitud inválida"),
        (status = 404, description = "Persona no encontrada"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_traits(
    Extension(ctx): Extension<ApiContext>,
    Path(persona_id): Path<i32>,
    Json(traits): Json<Traits>,
) -> Result<Response> {
    PersonaService::upsert_traits(&ctx, persona_id, traits).await?;

    Ok(StatusCode::CREATED.into_response())
}

/// Controlador para crear información de salud
#[utoipa::path(
    post,
    path = "",
    request_body = Health,
    params(
        ("id" = i32, Path, description = "ID de la persona")
    ),
    responses(
        (status = 201, description = "Información de salud creada exitosamente"),
        (status = 400, description = "Solicitud inválida"),
        (status = 404, description = "Persona no encontrada"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_health(
    Extension(ctx): Extension<ApiContext>,
    Path(persona_id): Path<i32>,
    Json(health): Json<Health>,
) -> Result<Response> {
    PersonaService::upsert_health(&ctx, persona_id, health).await?;

    Ok(StatusCode::CREATED.into_response())
}

/// Controlador para crear situación
#[utoipa::path(
    post,
    path = "",
    request_body = UpdateSituationDTO,
    params(
        ("id" = i32, Path, description = "ID de la persona")
    ),
    responses(
        (status = 201, description = "Situación creada exitosamente"),
        (status = 400, description = "Solicitud inválida"),
        (status = 404, description = "Persona no encontrada"),
        (status = 500, description = "Error interno del servidor")
    ),
    tag = PERSONA_TAG
)]
pub async fn create_situation(
    Extension(ctx): Extension<ApiContext>,
    Path(persona_id): Path<i32>,
    Json(situation): Json<UpdateSituationDTO>,
) -> Result<Response> {
    PersonaService::upsert_situation(&ctx, persona_id, situation).await?;

    Ok(StatusCode::CREATED.into_response())
}
