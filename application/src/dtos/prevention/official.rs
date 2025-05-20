use domain::entities::official;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOfficialDTO {
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: String,
    pub charge_id: i32,
    pub hierarchy_id: i32,
    pub brigade_id: i32,
}

impl Into<official::ActiveModel> for CreateOfficialDTO {
    fn into(self) -> official::ActiveModel {
        official::ActiveModel {
            last_name: Set(self.last_name),
            first_name: Set(self.first_name),
            phone: Set(self.phone),
            charge_id: Set(self.charge_id),
            hierarchy_id: Set(self.hierarchy_id),
            brigade_id: Set(self.brigade_id),
            ci: Set(self.ci),
            ..Default::default()
        }
    }
}
