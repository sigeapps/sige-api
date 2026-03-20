use crate::{tags::USER_TAG, Result};
use application::{
    dtos::{
        user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO, GetUserDTO, UpdateUserDTO},
        CommonQueryFilterDTO,
    },
    services::user::UserService,
};
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Extension, Json,
};
use domain::auth::permissions::Permission;
use serde::{Deserialize, Serialize};

use application::api::ApiContext;

#[derive(Serialize, Deserialize)]
pub struct UserBody {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ManyUsersBody {
    users: Vec<GetUserDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    user: Option<GetUserDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct RolesBody {
    roles: Vec<GetRoleDTO>,
}

pub async fn create_user(
    Extension(ctx): Extension<ApiContext>,
    Json(user): Json<CreateUserDTO>,
) -> Result<Response> {
    let id = UserService::create(ctx, user).await?;

    Ok(Json(UserBody { id }).into_response())
}

pub async fn get_users(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let users = UserService::find(ctx, query).await?;

    Ok(Json(ManyUsersBody { users }).into_response())
}

pub async fn get_user_by_id(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let user = UserService::find_by_id(ctx, id).await?;

    Ok(Json(UserResponse { user }).into_response())
}

pub async fn update_user(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUserDTO>,
) -> Result<Response> {
    let updated_id = UserService::update_user(ctx, user, id).await?;

    Ok(Json(UserBody { id: updated_id }).into_response())
}

#[utoipa::path(post, path = "", tag = USER_TAG,
    request_body = CreateRoleDTO,
    responses(
    (status = 201, description = "Rol creado de forma correcta"),
)
)]
pub async fn create_role(
    Extension(ctx): Extension<ApiContext>,
    Json(role): Json<CreateRoleDTO>,
) -> Result<Response> {
    UserService::create_role(ctx, role).await?;

    Ok(Json("Role created").into_response())
}

#[utoipa::path(get, path = "", tag = USER_TAG,
    params(
        CommonQueryFilterDTO
    ),
    responses(
    (status = 200, description = "Lista de permisos", body = [Permission]),
)
)]
pub async fn get_permissions(
    Extension(ctx): Extension<ApiContext>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let permissions = UserService::find_permissions(ctx, query).await?;

    Ok(Json(permissions).into_response())
}

#[utoipa::path(get, path = "", tag = USER_TAG,
    responses((status = 200, description = "Lista de roles", body = [GetRoleDTO]))
)]
pub async fn get_roles(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let roles = UserService::find_roles(ctx).await?;
    Ok(Json(roles).into_response())
}

#[utoipa::path(delete, path = "/{id}", tag = USER_TAG,
    params(("id" = i32, Path, description = "Role database id")),
    responses((status = 200, description = "Rol eliminado correctamente"))
)]
pub async fn delete_role(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    UserService::delete_role(ctx, id).await?;
    Ok(Json("Role deleted").into_response())
}
