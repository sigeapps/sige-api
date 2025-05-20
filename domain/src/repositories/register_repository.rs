use async_trait::async_trait;
use chrono::NaiveDate;
use sea_orm::{FromQueryResult, PartialModelTrait};

use crate::{entities::register, error::RepositoryError};

#[async_trait]
pub trait RegisterRepository {
    async fn create(&self, register: register::ActiveModel) -> Result<(), RepositoryError>;

    async fn find_by_id(&self, id: i32) -> Result<Option<register::Model>, RepositoryError>;

    async fn find(
        &self,
        search: Option<String>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<register::Model>, RepositoryError>;

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync;

    async fn update(&self, register: register::ActiveModel) -> Result<(), RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
