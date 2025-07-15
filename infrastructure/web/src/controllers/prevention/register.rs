use crate::tags::REGISTER_TAG;
use crate::Result;
use application::api::ApiContext;
use application::dtos::prevention::register::{
    CreateRegisterDTO, GetRegisterDTO, UpdateRegisterExitDTO,
};
use application::dtos::{CommonQueryFilterDTO, PaginationDTO};
use application::services::prevention::register::RegisterService;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistersBody {
    registers: Vec<GetRegisterDTO>,
    pagination: PaginationDTO,
}

#[utoipa::path(post, path = "", tag = REGISTER_TAG,
    request_body = CreateRegisterDTO,
    responses(
    (status = 201, description = "El registro se creo de forma correcta"),
)
)]
pub async fn create_register(
    Extension(ctx): Extension<ApiContext>,
    Json(register): Json<CreateRegisterDTO>,
) -> Result<Response> {
    RegisterService::create(ctx, register).await?;

    Ok((StatusCode::CREATED, "Register created successfully").into_response())
}

#[utoipa::path(get, path = "", tag = REGISTER_TAG,
    responses(
    (status = 200, description = "Listado de registros", body = RegistersBody),
),
 params(
            CommonQueryFilterDTO,
        ),
)]
#[axum::debug_handler]
pub async fn get_registers(
    Query(query): Query<CommonQueryFilterDTO>,

    Extension(ctx): Extension<ApiContext>,
) -> Result<Response> {
    let registers = RegisterService::find(ctx.clone(), query.clone()).await?;
    let pagination = RegisterService::get_pagination(ctx, query).await?;

    Ok((
        StatusCode::OK,
        Json(RegistersBody {
            registers,
            pagination,
        }),
    )
        .into_response())
}

#[utoipa::path(get, path = "/{id}", tag = REGISTER_TAG,
    params(
            ("id" = i32, Path, description = "ID del registro")
    ),
    responses(
    (status = 200, description = "Registro obtenido de forma correcta", body = GetRegisterDTO),
    (status = 404, description = "Registro no encontrado"),
    (status = 500, description = "Error en el servidor"),
)
)]
pub async fn get_register_by_id(
    Extension(ctx): Extension<ApiContext>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Response> {
    match RegisterService::find_by_id(ctx, id).await {
        Ok(Some(register)) => Ok((StatusCode::OK, Json(register)).into_response()),

        Ok(None) => Ok((StatusCode::NOT_FOUND, "Register not found").into_response()),

        Err(e) => {
            error!("Error fetching register: {}", e.to_string());

            Ok((StatusCode::INTERNAL_SERVER_ERROR, "Error fetching register").into_response())
        }
    }
}

#[utoipa::path(patch, path = "/{id}", tag = REGISTER_TAG,
    request_body = UpdateRegisterExitDTO,
    params(
            ("id" = i32, Path, description = "ID del registro")
    ),
    responses(
    (status = 200, description = "Registro actualizado de forma correcta"),
)
)]
#[axum::debug_handler]
pub async fn update_register_exit(
    Extension(ctx): Extension<ApiContext>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(register): Json<UpdateRegisterExitDTO>,
) -> Result<Response> {
    RegisterService::update_exit(ctx, register, id).await?;

    Ok((StatusCode::OK, "register updated successfully").into_response())
}
