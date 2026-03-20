use domain::entities::user::ActiveModel;
use sea_orm::{ActiveValue::Set, DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::prevention::lookup::GetBaseDTO;

use super::{personal::persona::SimplePersonaResponseDTO, prevention::lookup::GetHierarchyDTO};

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::user::Entity", from_query_result)]
pub struct GetUserDTO {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    #[sea_orm(nested)]
    pub base: GetBaseDTO,
    #[sea_orm(nested)]
    pub hierarchy: Option<GetHierarchyDTO>,
    #[sea_orm(nested)]
    pub persona: SimplePersonaResponseDTO,
    #[sea_orm(nested)]
    pub role: GetRoleDTO,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::role::Entity", from_query_result)]
pub struct GetRoleDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CreateRoleDTO {
    pub name: String,
    pub permissions: Vec<String>,
}

impl From<CreateRoleDTO> for domain::entities::role::ActiveModel {
    fn from(dto: CreateRoleDTO) -> Self {
        domain::entities::role::ActiveModel {
            name: Set(dto.name),
            ..Default::default()
        }
    }
}

#[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
pub struct CreateUserDTO {
    #[serde(rename(serialize = "username"))]
    pub name: String,
    pub persona_id: i32,
    #[serde(rename(serialize = "password"))]
    pub password_hash: String,
    pub role_id: i32,
}

#[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
pub struct UpdateUserDTO {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub role_id: i32,
}
