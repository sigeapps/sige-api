use crate::dtos::personal::country::AddVerificationDTO;
use domain::entities::country_verification;
use sea_orm::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CountryService {
    pub db: Arc<DatabaseConnection>,
}

impl CountryService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        CountryService { db }
    }

    pub async fn add_verification(&self, dto: AddVerificationDTO) -> Result<i32, DbErr> {
        let active_model: country_verification::ActiveModel = dto.into_active_model();

        let res = active_model.insert(&*self.db).await?;

        Ok(res.id)
    }
}
