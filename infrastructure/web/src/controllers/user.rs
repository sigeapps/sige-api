use crate::Result;
use application::dtos::user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO};
use axum::{
    extract::State,
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

pub async fn create_role(
    State(app_state): State<AppState>,
    Json(role): Json<CreateRoleDTO>,
) -> Result<Response> {
    app_state.user_service.create_role(role).await?;

    Ok(Json("Role created").into_response())
}
