use std::sync::Arc;

use sea_orm::*;

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
}
