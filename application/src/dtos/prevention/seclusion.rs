use chrono::NaiveDate;
use domain::entities::seclusion::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::temporal_seclusion::Entity", from_query_result)]
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
    pub exit_at: NaiveDate,
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
