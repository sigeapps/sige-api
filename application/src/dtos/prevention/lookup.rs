use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: Encontrar forma de evitar duplicación de código

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct LookupItemDto {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::state::Entity", from_query_result)]
pub struct GetStateDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::base::Entity", from_query_result)]
pub struct GetBaseDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::institution::Entity", from_query_result)]
pub struct GetInstitutionDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::organism::Entity", from_query_result)]
pub struct GetOrganismDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::brand::Entity", from_query_result)]
pub struct GetBrandDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::vehicle_model::Entity", from_query_result)]
pub struct GetVehicleModelDTO {
    pub id: i32,
    pub name: String,
    #[sea_orm(nested)]
    pub brand: GetBrandDTO,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(
    entity = "domain::entities::transport_statuses::Entity",
    from_query_result
)]
pub struct GetTransportStatusDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::transport_type::Entity", from_query_result)]
pub struct GetTransportTypeDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(
    entity = "domain::entities::seclusion_statuses::Entity",
    from_query_result
)]
pub struct GetSeclusionStatusDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::brigade::Entity", from_query_result)]
pub struct GetBrigadeDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::charge::Entity", from_query_result)]
pub struct GetChargeDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::hierarchy::Entity", from_query_result)]
pub struct GetHierarchyDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::municipality::Entity", from_query_result)]
pub struct GetMunicipalityDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::parish::Entity", from_query_result)]
pub struct GetParishDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(
    entity = "domain::entities::family_relationship::Entity",
    from_query_result
)]
pub struct GetFamilyRelationshipDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::novelty::Entity", from_query_result)]
pub struct GetNoveltyDTO {
    pub id: i32,
    pub name: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::division::Entity", from_query_result)]
pub struct GetSimpleDivisionDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::division::Entity", from_query_result)]
pub struct GetDivisionDTO {
    pub id: i32,
    pub name: String,
    #[sea_orm(nested)]
    pub state: GetStateDTO,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel)]
#[sea_orm(
    entity = "domain::entities::status_condition::Entity",
    from_query_result
)]
pub struct GetStatusConditionDTO {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DerivePartialModel, ToSchema)]
#[sea_orm(entity = "domain::entities::persona_state::Entity", from_query_result)]
pub struct GetPersonaStateDTO {
    pub id: i32,
    pub name: String,
}
