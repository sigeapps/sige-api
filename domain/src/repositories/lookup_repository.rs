use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, FromQueryResult, ModelTrait, PartialModelTrait};

use crate::error::RepositoryError;

#[async_trait]
pub trait LookupRepository {
    async fn find<E, M, A>(&self) -> Result<Vec<M>, RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait + Send + Sync;

    async fn create<E, M, A>(&self, active_model: A) -> Result<(), RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync;

    async fn find_partial<E, M, A, P>(&self) -> Result<Vec<P>, RepositoryError>
    where
        E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
        M: ModelTrait + Send + Sync + FromQueryResult,
        A: ActiveModelTrait<Entity = E> + Send + Sync,
        P: PartialModelTrait + FromQueryResult + Send + Sync;
}
