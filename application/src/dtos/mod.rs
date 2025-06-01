use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod auth;
pub mod lookup;
pub mod prevention;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CommonQueryFilterDTO {
    pub search: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

impl CommonQueryFilterDTO {
    pub fn into_pagination(self) -> PaginationDTO {
        let limit = self.limit.unwrap_or(20);
        let page = self.page.unwrap_or(1);

        PaginationDTO {
            page,
            limit,
            page_count: 0,
            total_count: 0,
            offset: ((page - 1) * limit),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDTO {
    pub page: u64,
    pub limit: u64,
    pub page_count: u64,
    pub total_count: u64,
    #[serde(skip_deserializing)]
    pub(crate) offset: u64,
}
