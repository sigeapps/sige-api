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
    pub ci: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub sort: Option<String>,
}

impl CommonQueryFilterDTO {
    pub fn into_pagination(&self) -> PaginationDTO {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        PaginationDTO {
            page,
            limit,
            page_count: 0,
            total_count: 0,
            offset,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDTO {
    pub page: i32,
    pub limit: i32,
    pub page_count: i32,
    pub total_count: i32,
    pub offset: i32,
}
