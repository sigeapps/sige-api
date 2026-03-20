use domain::entities::{brand, division, state, vehicle_model};
use sea_orm::*;
use serde::Serialize;

use crate::{
    api::ApiContext,
    dtos::prevention::lookup::{GetDivisionDTO, GetVehicleModelDTO},
};

#[derive(Debug, Clone)]
pub struct LookupService {}

impl LookupService {
    pub async fn find<E, M, A>(ctx: ApiContext) -> Result<Vec<M>, DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult + Serialize + for<'de> serde::Deserialize<'de> + 'static,
        A: ActiveModelTrait + Send + Sync,
    {
        let key = format!("lookup:{}", E::default().table_name());
        crate::services::cache_manager::CacheManager::get_or_set(&ctx.cache, &key, || async {
            E::find().all(&ctx.db).await
        })
        .await
    }

    pub async fn create<E, M, A>(ctx: ApiContext, active_model: A) -> Result<(), DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
    {
        E::insert(active_model).exec(&ctx.db).await?;

        // Invalida la caché de esta tabla para que se refresque en la próxima consulta
        let key = format!("lookup:{}", E::default().table_name());
        crate::services::cache_manager::CacheManager::invalidate(&ctx.cache, &key).await;

        Ok(())
    }

    pub async fn find_vehicle_models(ctx: ApiContext) -> Result<Vec<GetVehicleModelDTO>, DbErr> {
        let vehicle_models = vehicle_model::Entity::find()
            .left_join(brand::Entity)
            .into_partial_model::<GetVehicleModelDTO>()
            .all(&ctx.db)
            .await?;

        Ok(vehicle_models)
    }

    pub async fn find_divisions(ctx: ApiContext) -> Result<Vec<GetDivisionDTO>, DbErr> {
        let divisions = division::Entity::find()
            .left_join(state::Entity)
            .into_partial_model::<GetDivisionDTO>()
            .all(&ctx.db)
            .await?;

        Ok(divisions)
    }
}
