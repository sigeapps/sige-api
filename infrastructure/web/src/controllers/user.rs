use crate::Result;
use application::dtos::{
    user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO, GetUserDTO, UpdateUserDTO},
    CommonQueryFilterDTO,
};
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

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
    State(app_state): State<AppState>,
    Json(user): Json<CreateUserDTO>,
) -> Result<Response> {
    let id = app_state.user_service.create(user).await?;

    Ok(Json(UserBody { id }).into_response())
}

pub async fn get_users(
    State(app_state): State<AppState>,
    Query(query): Query<CommonQueryFilterDTO>,
) -> Result<Response> {
    let users = app_state.user_service.find(query).await?;

    Ok(Json(ManyUsersBody { users }).into_response())
}

pub async fn get_user_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let user = app_state.user_service.find_by_id(id).await?;

    Ok(Json(UserResponse { user }).into_response())
}

pub async fn update_user(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUserDTO>,
) -> Result<Response> {
    let updated_id = app_state.user_service.update_user(user, id).await?;

    Ok(Json(UserBody { id: updated_id }).into_response())
}

pub async fn create_role(
    State(app_state): State<AppState>,
    Json(role): Json<CreateRoleDTO>,
) -> Result<Response> {
    app_state.user_service.create_role(role).await?;

    Ok(Json("Role created").into_response())
}
