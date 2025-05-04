use async_trait::async_trait;
use domain::entities::{prelude::*, *};
use domain::error::RepositoryError;
use domain::repositories::lookup_repository::LookupRepository;
use domain::repositories::register_repository::RegisterRepository;
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
impl<E, M, A> LookupRepository<E, M, A> for SeaOrmLookupRepository
where
    E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
    M: ModelTrait + Send + Sync,
    A: ActiveModelTrait + Send + Sync,
{
    async fn find(&self) -> Result<Vec<M>, RepositoryError> {
        E::find()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
}
