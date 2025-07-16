use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub mod auth;
pub mod lookup;
pub mod parking;
pub mod personal;
pub mod prevention;
pub mod user;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithId<T, ID = i32> {
    pub id: ID,
    #[serde(flatten)]
    pub data: T,
}

// TODO: Agregar validaciones y moverlo a api.rs
#[derive(Debug, Serialize, Deserialize, Validate, Clone, IntoParams)]
pub struct CommonQueryFilterDTO {
    pub search: Option<String>,
    pub ci: Option<String>,
    #[param(allow_reserved, value_type = String, example = "2022-01-22", format = Date)]
    pub from_date: Option<NaiveDate>,
    #[param(allow_reserved, value_type = String, example = "2022-01-22", format = Date)]
    pub to_date: Option<NaiveDate>,
    // Se utiliza para filtrar por estado de la operacion, casi todos los servicios pueden utilizar esta propiedad
    pub finalized: Option<bool>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationDTO {
    pub page: u64,
    pub limit: u64,
    pub page_count: u64,
    pub total_count: u64,
    pub offset: u64,
}
