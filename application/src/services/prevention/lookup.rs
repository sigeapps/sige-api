use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, ModelTrait,
};

use crate::connection::connect;

#[derive(Debug, Clone)]
pub struct LookupService {
    db: DatabaseConnection,
}

impl LookupService {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = connect(db_url).await?;

        Ok(LookupService { db })
    }

    pub async fn find<E, M, A>(&self) -> Result<Vec<M>, DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait + Send + Sync,
    {
        E::find().all(&self.db).await
    }

    pub async fn create<E, M, A>(&self, active_model: A) -> Result<(), DbErr>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
    {
        E::insert(active_model).exec(&self.db).await?;

        Ok(())
    }
}
