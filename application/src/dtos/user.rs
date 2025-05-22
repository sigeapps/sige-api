use domain::entities::prelude::User;
use domain::entities::user;
use domain::entities::user::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "User")]
pub struct GetUserDTO {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub role_id: i32,
}

impl From<user::Model> for GetUserDTO {
    fn from(user: user::Model) -> Self {
        GetUserDTO {
            id: user.id,
            name: user.name,
            password_hash: user.password_hash,
            role_id: user.role_id,
        }
    }
}

#[derive(DeriveIntoActiveModel, Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub name: String,
    #[serde(rename(serialize = "password"))]
    pub password_hash: String,
}
