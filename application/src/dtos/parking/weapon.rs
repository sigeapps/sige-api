use chrono::NaiveDateTime;
use domain::entities::calibre;
use domain::entities::weapon::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
pub struct WeaponCreate {
    pub type_id: i32,
    pub model_id: i32,
    pub photo: Option<String>,
    pub serial: String,
    #[schema(value_type = String, format = Date)]
    pub entry_at: NaiveDateTime,
    pub document_id: Option<String>,
    pub calibre_id: i32,
    pub position_id: i32,
    #[schema(value_type = String, format = Date)]
    pub manteinance_at: NaiveDateTime,
    pub has_charger: bool,
    pub observations: Option<String>,
}

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct WeaponView {
    pub id: i32,
    pub r#type: String,
    pub model: String,
    pub photo: Option<String>,
    pub serial: String,
    #[schema(value_type = String, format = Date)]
    pub entry_at: NaiveDateTime,
    pub document_id: Option<String>,
    pub calibre: String,
    #[schema(value_type = String, format = Date)]
    pub manteinance_at: Option<NaiveDateTime>,
    pub has_charger: bool,
    pub observations: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, DerivePartialModel)]
#[sea_orm(entity = "domain::entities::weapon::Entity", from_query_result)]
pub struct WeaponSummary {
    pub id: i32,
    pub serial: String,
    #[sea_orm(from_expr = "calibre::Column::Id")]
    pub calibre: String,
    pub has_charger: bool,
}
