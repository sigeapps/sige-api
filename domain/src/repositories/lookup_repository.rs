use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, EntityTrait, FromQueryResult, IntoActiveValue, ModelTrait, PartialModelTrait,
};

use crate::{entities::municipality::ActiveModel, error::RepositoryError};

/// R Generics refers to lookup::Model, E refers to lookup::ActiveModel
#[async_trait]
pub trait LookupRepository<E, M, A>
where
    E: EntityTrait<Model = M, ActiveModel = A> + Send + Sync,
    M: ModelTrait + Send + Sync + FromQueryResult,
    A: ActiveModelTrait + Send + Sync,
{
    async fn find(&self) -> Result<Vec<M>, RepositoryError>;
}
