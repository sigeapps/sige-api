use domain::entities::transport;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTransportDTO {
    pub type_id: i32,
    pub details: String,
    pub brand_id: i32,
    pub model_id: i32,
    pub plate: String,
    pub unit: String,
    pub status_id: Option<i32>,
}

impl Into<transport::ActiveModel> for CreateTransportDTO {
    fn into(self) -> transport::ActiveModel {
        transport::ActiveModel {
            plate: Set(self.plate),
            unit: Set(self.unit),
            details: Set(self.details),
            type_id: Set(self.type_id),
            brand_id: Set(self.brand_id),
            model_id: Set(self.model_id),
            status_id: Set(self.status_id),
            ..Default::default()
        }
    }
}
