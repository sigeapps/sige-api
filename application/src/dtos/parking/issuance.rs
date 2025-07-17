use chrono::NaiveDateTime;
use domain::entities::issuance::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::{parking::weapon::WeaponSummary, personal::persona::SimplePersonaResponseDTO};

#[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
pub struct StartIssuance {
    pub assigned_persona_id: i32,
    pub assigned_weapon_id: i32,
    pub has_charger: bool,
    pub ammo_count: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub r#type: String,
    pub assignance_days: i32,
    pub auth_by_id: i32,
}

#[derive(Deserialize, Serialize, ToSchema, FromQueryResult)]
pub struct IssuanceSummary {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub assignance_days: i32,
    #[schema(value_type = String, format = Date)]
    pub returned_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::issuance::Entity", from_query_result)]
pub struct IssuanceView {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub assignance_days: i32,
    #[schema(value_type = String, format = Date)]
    #[sea_orm(skip)]
    pub returned_at: Option<NaiveDateTime>,
    #[sea_orm(nested)]
    pub assigned_weapon: WeaponSummary,
    #[sea_orm(nested)]
    pub assigned_persona: SimplePersonaResponseDTO,
    pub r#type: String,
}

pub mod returns {
    use chrono::NaiveDateTime;
    use domain::entities::issuance_return::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
    pub struct FinalizeIssuance {
        pub issuance_id: i32,
        #[schema(value_type = String, format = Date)]
        pub returned_at: NaiveDateTime,
        pub has_charger: bool,
        pub returned_ammo: i32,
        pub observations: Option<String>,
    }
}
