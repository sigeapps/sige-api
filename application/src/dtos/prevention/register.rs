use domain::entities::register;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterExit {
    pub observations: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateRegister {
    pub photo: Option<String>,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub organism_id: Option<i32>,
    pub division_id: Option<i32>,
    pub is_official: Option<bool>,
    pub visit_reason: String,
    pub observations: Option<String>,
}

impl Into<register::ActiveModel> for CreateRegister {
    fn into(self) -> register::ActiveModel {
        register::ActiveModel {
            photo: Set(self.photo),
            ci: Set(self.ci),
            last_name: Set(self.last_name),
            first_name: Set(self.first_name),
            organism: Set(self.organism_id),
            division: Set(self.division_id),
            is_official: Set(self.is_official.unwrap_or(false)),
            visit_reason: Set(self.visit_reason),
            observations: Set(self.observations),
            ..Default::default()
        }
    }
}
