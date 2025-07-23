use chrono::NaiveDateTime;
use domain::entities::transport_issuance::ActiveModel;
use domain::entities::transport_issuance_return;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dtos::{
    personal::persona::SimplePersonaResponseDTO, transport::transport::TransportSummary,
};

#[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
pub struct StartTransportIssuance {
    pub assigned_persona_id: i32,
    pub assigned_transport_id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub r#type: String,
    pub assignance_days: i32,
    pub auth_by_id: i32,
}

#[derive(Deserialize, Serialize, ToSchema, FromQueryResult)]
pub struct TransportIssuanceSummary {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub assignance_days: i32,
    #[schema(value_type = String, format = Date)]
    pub returned_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, ToSchema, DerivePartialModel)]
#[sea_orm(
    entity = "domain::entities::transport_issuance::Entity",
    from_query_result
)]
pub struct TransportIssuanceView {
    pub id: i32,
    #[schema(value_type = String, format = Date)]
    pub date_time: NaiveDateTime,
    pub assignance_days: i32,
    #[schema(value_type = String, format = Date)]
    #[sea_orm(from_expr = "transport_issuance_return::Column::ReturnedAt")]
    pub returned_at: Option<NaiveDateTime>,
    #[sea_orm(nested)]
    pub assigned_transport: TransportSummary,
    #[sea_orm(nested)]
    pub assigned_persona: SimplePersonaResponseDTO,
    pub r#type: String,
}

pub mod returns {
    use chrono::NaiveDateTime;
    use domain::entities::transport_issuance_return::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Deserialize, Serialize, ToSchema, DeriveIntoActiveModel)]
    pub struct FinalizeTransportIssuance {
        pub issuance_id: i32,
        #[schema(value_type = String, format = Date)]
        pub returned_at: NaiveDateTime,
        pub observations: Option<String>,
    }
}
