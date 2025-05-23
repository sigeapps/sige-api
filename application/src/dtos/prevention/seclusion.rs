use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct GetSeclusionDTO {
    pub id: i32,
    pub commission_id: i32,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
}
