use chrono::NaiveDate;
use domain::entities::seclusion::{self, ActiveModel};
use sea_orm::{prelude::DateTimeUtc, DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};
use visit::GetSeclusionVisitDTO;

#[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
#[sea_orm(
    entity = "domain::entities::temporal_seclusion::Entity",
    from_query_result
)]
pub struct GetTemporalSeclusionDTO {
    pub id: i32,
    pub commission_id: i32,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::seclusion::Entity", from_query_result)]
pub struct GetSeclusionDTO {
    pub id: i32,
    pub photo: Option<String>,
    pub ci: String,
    pub birthdate: NaiveDate,
    pub age: i32,
    pub last_name: String,
    pub first_name: String,
    pub reason: String,
    pub exit_reason: Option<String>,
    pub physical_state: String,
    pub outfit: Option<String>,
    pub belongings: Option<String>,
    pub observations: Option<String>,
    pub exit_at: Option<NaiveDate>,
}

impl From<seclusion::Model> for GetSeclusionDTO {
    fn from(model: seclusion::Model) -> Self {
        GetSeclusionDTO {
            id: model.id,
            photo: model.photo,
            ci: model.ci,
            birthdate: model.birthdate,
            age: model.age,
            last_name: model.last_name,
            first_name: model.first_name,
            reason: model.reason,
            exit_reason: model.exit_reason,
            physical_state: model.physical_state,
            outfit: model.outfit,
            belongings: model.belongings,
            observations: model.observations,
            exit_at: model.exit_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSeclusionWithVisitDTO {
    #[serde(flatten)]
    pub seclusion: GetSeclusionDTO,
    pub visits: Vec<GetSeclusionVisitDTO>,
}

#[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct CreateSeclusionDTO {
    pub photo: String,
    pub ci: String,
    pub birthdate: NaiveDate,
    pub age: i32,
    pub last_name: String,
    pub first_name: String,
    pub reason: String,
    pub physical_state: String,
    pub outfit: String,
    pub belongings: String,
    pub observations: String,
}

#[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct UpdateSeclusionExitDTO {
    #[serde(skip_deserializing)]
    pub id: i32,
    exit_at: NaiveDate,
    observations: String,
}

pub mod visit {
    use domain::entities::seclusion_visit::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    use crate::dtos::prevention::lookup::GetFamilyRelationshipDTO;

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct AddSeclusionVisitDTO {
        #[serde(skip_deserializing)]
        pub seclusion_id: i32,
        pub ci: String,
        pub last_name: String,
        pub first_name: String,
        pub relationship_id: i32,
        pub phone: String,
        pub address: String,
        pub reason: String,
    }

    #[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
    #[sea_orm(
        entity = "domain::entities::seclusion_visit::Entity",
        from_query_result
    )]
    pub struct GetSeclusionVisitDTO {
        pub id: i32,
        pub seclusion_id: i32,
        pub ci: String,
        pub last_name: String,
        pub first_name: String,
        #[sea_orm(nested)]
        pub relationship: GetFamilyRelationshipDTO,
        pub phone: String,
        pub address: String,
        pub reason: Option<String>,
    }
}
