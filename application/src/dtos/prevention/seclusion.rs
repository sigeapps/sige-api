use chrono::NaiveDate;
use domain::entities::seclusion::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
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
    pub exit_reason: String,
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
    use chrono::NaiveDate;
    use domain::entities::seclusion_visit::ActiveModel;
    use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
    pub struct AddSeclusionVisitDTO {
        #[serde(skip_deserializing)]
        pub seclusion_id: i32,
        pub ci: String,
        pub last_name: String,
        pub first_name: String,
        pub relationship_id: i32,
        pub phone: String,
        pub date: NaiveDate,
        pub address: NaiveDate,
        pub reason: NaiveDate,
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
        pub relationship_id: i32,
        pub phone: String,
        pub date: NaiveDate,
        pub address: NaiveDate,
        pub reason: Option<NaiveDate>,
    }
}
