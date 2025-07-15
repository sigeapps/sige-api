use domain::entities::weapon::ActiveModel;
use sea_orm::{prelude::DateTimeWithTimeZone, DeriveIntoActiveModel, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
pub struct WeaponCreate {
    pub type_id: i32,
    pub model_id: i32,
    pub photo: Option<String>,
    pub serial: String,
    #[schema(value_type = String, format = Date)]
    pub entry_at: DateTimeWithTimeZone,
    pub document_id: Option<String>,
    pub calibre: String,
    #[schema(value_type = String, format = Date)]
    pub manteinance_at: Option<DateTimeWithTimeZone>,
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
    pub entry_at: DateTimeWithTimeZone,
    pub document_id: Option<String>,
    pub calibre: String,
    #[schema(value_type = String, format = Date)]
    pub manteinance_at: Option<DateTimeWithTimeZone>,
    pub has_charger: bool,
    pub observations: Option<String>,
}
