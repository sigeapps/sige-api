use domain::entities::official::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, DeriveIntoActiveModel)]
pub struct CreateOfficialDTO {
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: String,
    pub charge_id: i32,
    pub hierarchy_id: i32,
    pub brigade_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct GetOfficialDTO {
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: String,
    pub charge: String,
    pub hierarchy: String,
    pub brigade: String,
    pub code: Option<i32>,
}
