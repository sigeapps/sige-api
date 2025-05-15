use async_trait::async_trait;
use domain::error::RepositoryError;
use domain::repositories::lookup_repository::LookupRepository;
use sea_orm::*;

use crate::connection::connect;

#[derive(Clone, Debug)]
pub struct SeaOrmLookupRepository {
    db: DatabaseConnection,
}

impl SeaOrmLookupRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self { db })
    }
}

#[async_trait]
impl LookupRepository for SeaOrmLookupRepository {
    async fn find<E, M, A>(&self) -> Result<Vec<M>, RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait + Send + Sync,
    {
        E::find()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    async fn create<E, M, A>(&self, active_model: A) -> Result<(), RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
    {
        E::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
    async fn find_partial<E, M, A, P>(&self) -> Result<Vec<P>, RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
        P: PartialModelTrait + FromQueryResult + Send + Sync,
    {
        E::find()
            .into_partial_model::<P>()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
}
