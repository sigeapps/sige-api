use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod auth;
pub mod lookup;
pub mod prevention;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CommonQueryFilterDTO {
    pub search: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
