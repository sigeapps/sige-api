use domain::entities::transport::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, DeriveIntoActiveModel)]
pub struct CreateTransportDTO {
    pub type_id: i32,
    pub details: String,
    pub brand_id: i32,
    pub model_id: i32,
    pub plate: String,
    pub unit: String,
    pub status_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct GetTransportDTO {
    pub id: i32,
    pub r#type: String,
    pub details: String,
    pub brand: String,
    pub model: String,
    pub plate: String,
    pub unit: String,
    pub status: String,
}
