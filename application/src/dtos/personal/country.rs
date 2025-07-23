use domain::entities::country_verification::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct AddVerificationDTO {
    pub persona_id: i32,
    pub condition_id: i32,
    pub reported: bool,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug, ToSchema)]
#[sea_orm(
    entity = "domain::entities::country_verification::Entity",
    from_query_result
)]
pub struct GetVerificationDTO {
    pub condition_id: i32,
    pub reported: bool,
}

#[derive(Serialize, Deserialize, DerivePartialModel, Clone, Debug, ToSchema)]
#[sea_orm(entity = "domain::entities::persona::Entity", from_query_result)]
pub struct GetPersonaCSV {
    pub ci: String,
    pub genre: String,
}
