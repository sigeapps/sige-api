use domain::entities::official::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

use super::lookup::{GetBrigadeDTO, GetChargeDTO, GetHierarchyDTO};

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

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::official::Entity", from_query_result)]
pub struct GetOfficialDTO {
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: String,
    #[sea_orm(nested)]
    pub charge: GetChargeDTO,
    #[sea_orm(nested)]
    pub hierarchy: GetHierarchyDTO,
    #[sea_orm(nested)]
    pub brigade: GetBrigadeDTO,
    pub code: Option<i32>,
}
