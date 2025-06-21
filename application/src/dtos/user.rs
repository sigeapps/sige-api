use domain::entities::user::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::user::Entity", from_query_result)]
pub struct GetUserDTO {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    #[sea_orm(nested)]
    pub role: GetRoleDTO,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::role::Entity", from_query_result)]
pub struct GetRoleDTO {
    pub id: i32,
    pub name: String,
}

#[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
pub struct CreateUserDTO {
    #[serde(rename(serialize = "username"))]
    pub name: String,
    #[serde(rename(serialize = "password"))]
    pub password_hash: String,
    pub role_id: i32,
}
