use crate::{api::ApiContext, dtos::personal::country::AddVerificationDTO};
use domain::entities::country_verification;
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct CountryService {}

impl CountryService {
    pub async fn add_verification(ctx: ApiContext, dto: AddVerificationDTO) -> Result<i32, DbErr> {
        let active_model: country_verification::ActiveModel = dto.into_active_model();

        let res = active_model.insert(&ctx.db).await?;

        Ok(res.id)
    }
}
