use chrono::NaiveDateTime;
use domain::entities::issuance::ActiveModel;
use domain::entities::issuance_return;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::{parking::weapon::WeaponSummary, personal::persona::SimplePersonaResponseDTO};

#[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
pub struct StartIssuanceBase {
    pub assigned_persona_id: i32,
    #[schema(value_type = String, format = Date, example = "2023-10-27T10:00:00Z")]
    pub date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date, example = "2023-10-27T10:00:00Z")]
    pub returned_date_time: NaiveDateTime,
    pub r#type: String,
    pub auth_by_id: i32,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct StartIssuance {
    #[serde(flatten)]
    pub base: StartIssuanceBase,
    pub assigned_weapons_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, ToSchema, FromQueryResult)]
pub struct IssuanceSummary {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub returned_date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub returned_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::issuance::Entity", from_query_result)]
pub struct IssuanceView {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub returned_date_time: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    #[sea_orm(from_expr = "issuance_return::Column::ReturnedAt")]
    pub returned_at: Option<NaiveDateTime>,
    #[sea_orm(skip)]
    pub weapons: Vec<WeaponSummary>,
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
        pub observations: Option<String>,
    }
}
