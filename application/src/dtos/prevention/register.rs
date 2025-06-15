use chrono::NaiveDateTime;
use domain::entities::register::ActiveModel;
use domain::entities::{division, organism, register};
use sea_orm::DeriveIntoActiveModel;
use serde::{Deserialize, Serialize};

use super::lookup::LookupItemDto;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterExit {
    pub observations: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, DeriveIntoActiveModel)]
pub struct CreateRegisterDTO {
    pub photo: Option<String>,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub organism: Option<i32>,
    pub division: Option<i32>,
    pub is_official: bool,
    pub visit_reason: String,
    pub observations: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRegisterDTO {
    pub id: i32,
    pub photo: Option<String>,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub organism: Option<LookupItemDto>,
    pub division: Option<LookupItemDto>,
    pub entry_date: NaiveDateTime,
    pub exit_date: Option<NaiveDateTime>,
    pub is_official: Option<bool>,
    pub visit_reason: String,
    pub observations: Option<String>,
}

impl
    From<(
        register::Model,
        Option<organism::Model>,
        Option<division::Model>,
    )> for GetRegisterDTO
{
    fn from(
        r: (
            register::Model,
            Option<organism::Model>,
            Option<division::Model>,
        ),
    ) -> Self {
        GetRegisterDTO {
            id: r.0.id,
            photo: r.0.photo,
            ci: r.0.ci,
            last_name: r.0.last_name,
            first_name: r.0.first_name,
            organism: r.1.map(|o| LookupItemDto {
                id: o.id,
                name: o.name,
            }),
            division: r.2.map(|d| LookupItemDto {
                id: d.id,
                name: d.name,
            }),
            is_official: Some(r.0.is_official),
            visit_reason: r.0.visit_reason,
            entry_date: r.0.entry_date,
            exit_date: r.0.exit_date,
            observations: r.0.observations,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct UpdateRegisterExitDTO {
    pub observations: String,
}
