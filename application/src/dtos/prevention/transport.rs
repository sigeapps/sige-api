use domain::entities::transport::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::lookup::{GetBrandDTO, GetTransportStatusDTO, GetTransportTypeDTO, GetVehicleModelDTO};

#[derive(Debug, Clone, Deserialize, Serialize, DeriveIntoActiveModel, ToSchema)]
pub struct CreateTransportDTO {
    pub type_id: i32,
    pub details: Option<String>,
    pub brand_id: i32,
    pub model_id: i32,
    pub plate: String,
    pub unit: String,
    pub status_id: Option<i32>,
    pub serial_photo: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::transport::Entity", from_query_result)]
pub struct GetTransportDTO {
    pub id: i32,
    #[sea_orm(nested)]
    pub r#type: GetTransportTypeDTO,
    pub details: Option<String>,
    #[sea_orm(nested)]
    pub brand: GetBrandDTO,
    #[sea_orm(nested)]
    pub model: GetVehicleModelDTO,
    pub plate: String,
    pub unit: String,
    #[sea_orm(nested)]
    pub status: Option<GetTransportStatusDTO>,
}
