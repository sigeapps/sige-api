use chrono::NaiveDateTime;
use domain::entities::issuance::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
pub struct StartIssuance {
    pub assigned_persona_id: i32,
    pub assigned_weapon_id: i32,
    pub has_charger: bool,
    pub ammo_count: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub reason: String,
    pub r#type: String,
    #[schema(value_type = String, format = Date)]
    pub assignance_time: NaiveDateTime,
    pub auth_by_id: i32,
}

#[derive(Deserialize, Serialize, ToSchema, FromQueryResult)]
pub struct IssuanceSummary {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub assignance_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub returned_at: Option<NaiveDateTime>,
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
