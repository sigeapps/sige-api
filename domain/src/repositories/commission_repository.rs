use async_trait::async_trait;
use sea_orm::{FromQueryResult, PartialModelTrait};

use crate::{
    entities::{commission, commission_official, commission_reason, commission_transport},
    error::RepositoryError,
};

pub struct CommissionAggregate {
    pub commission: commission::ActiveModel,
    pub reason: commission_reason::ActiveModel,
    pub officials: Vec<commission_official::ActiveModel>,
    pub transports: Vec<commission_transport::ActiveModel>,
}

#[async_trait]
pub trait CommissionAggregateRepository {
    async fn create_with_relations(
        &self,
        data: CommissionAggregate,
    ) -> Result<i32, RepositoryError>;

    async fn find_with_relations(&self, id: i32) -> Result<CommissionAggregate, RepositoryError>;
}

#[async_trait]
pub trait CommissionRepository {
    async fn create(&self, data: commission::ActiveModel) -> Result<i32, RepositoryError>;

    async fn find(
        &self,
        search: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<commission::Model>, RepositoryError>;

    async fn find_by_id(&self, id: i32) -> Result<commission::Model, RepositoryError>;

    async fn update(
        &self,
        id: i32,
        commission: commission::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync;
}

#[async_trait]
pub trait CommissionReasonRepository {
    async fn create(
        &self,
        data: crate::entities::commission_reason::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<crate::entities::commission_reason::Model, RepositoryError>;

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<crate::entities::commission_reason::Model>, RepositoryError>;

    async fn update(
        &self,
        id: i32,
        data: crate::entities::commission_reason::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync;
}

#[async_trait]
pub trait CommissionOfficialRepository {
    async fn create(
        &self,
        data: crate::entities::commission_official::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<crate::entities::commission_official::Model, RepositoryError>;

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<crate::entities::commission_official::Model>, RepositoryError>;

    async fn update(
        &self,
        id: i32,
        data: crate::entities::commission_official::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync;
}

#[async_trait]
pub trait CommissionTransportRepository {
    async fn create(
        &self,
        data: crate::entities::commission_transport::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<crate::entities::commission_transport::Model, RepositoryError>;

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<crate::entities::commission_transport::Model>, RepositoryError>;

    async fn update(
        &self,
        id: i32,
        data: crate::entities::commission_transport::ActiveModel,
    ) -> Result<i32, RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync;
}
