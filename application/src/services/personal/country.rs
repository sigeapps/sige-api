use crate::{
    api::ApiContext,
    dtos::personal::country::{AddVerificationDTO, GetPersonaCSV},
};
use domain::entities::{country_verification, persona};
use sea_orm::{prelude::*, *};

#[derive(Debug, Clone)]
pub struct CountryService {}

impl CountryService {
    pub async fn add_verification(ctx: ApiContext, dto: AddVerificationDTO) -> Result<i32, DbErr> {
        let active_model: country_verification::ActiveModel = dto.into_active_model();

        let res = active_model.insert(&ctx.db).await?;

        Ok(res.id)
    }

    pub async fn get_country_csv(ctx: ApiContext) -> Result<Vec<GetPersonaCSV>, DbErr> {
        // Usaremos una query cruda ya que sea_orm no soporta el select con cases

        let personas = persona::Entity::find()
            .from_raw_sql(Statement::from_string(
                DatabaseBackend::Postgres,
                "
SELECT ci, CASE WHEN genre = 'male' THEN 'm' WHEN genre = 'female' THEN 'f' ELSE '?' END AS genre FROM persona
",
            ))
            .into_model::<GetPersonaCSV>()
            .all(&ctx.db)
            .await?;

        Ok(personas)
    }
}
