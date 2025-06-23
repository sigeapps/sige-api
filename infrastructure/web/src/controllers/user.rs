use crate::Result;
use application::dtos::{
    user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO, GetUserDTO},
    CommonQueryFilterDTO,
};
use axum::{
    extract::{Query, State},
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

pub async fn create_role(
    State(app_state): State<AppState>,
    Json(role): Json<CreateRoleDTO>,
) -> Result<Response> {
    app_state.user_service.create_role(role).await?;

    Ok(Json("Role created").into_response())
}
