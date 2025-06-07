use std::sync::Arc;

use domain::entities::{brand, division, state, vehicle_model};
use sea_orm::*;

use crate::dtos::prevention::lookup::{GetDivisionDTO, GetVehicleModelDTO};

#[derive(Debug, Clone)]
pub struct LookupService {
    db: Arc<DatabaseConnection>,
}

impl LookupService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        LookupService { db }
    }

    pub async fn find<E, M, A>(&self) -> Result<Vec<M>, DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait + Send + Sync,
    {
        E::find().all(&*self.db).await
    }

    pub async fn create<E, M, A>(&self, active_model: A) -> Result<(), DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
    {
        E::insert(active_model).exec(&*self.db).await?;

        Ok(())
    }

    pub async fn find_vehicle_models(&self) -> Result<Vec<GetVehicleModelDTO>, DbErr> {
        let vehicle_models = vehicle_model::Entity::find()
            .left_join(brand::Entity)
            .into_partial_model::<GetVehicleModelDTO>()
            .all(&*self.db)
            .await?;

        Ok(vehicle_models)
    }

    pub async fn find_divisions(&self) -> Result<Vec<GetDivisionDTO>, DbErr> {
        let divisions = division::Entity::find()
            .left_join(state::Entity)
            .into_partial_model::<GetDivisionDTO>()
            .all(&*self.db)
            .await?;

        Ok(divisions)
    }
}
