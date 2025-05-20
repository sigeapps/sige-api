use async_trait::async_trait;
use domain::entities::{prelude::*, *};
use domain::repositories::commission_repository::CommissionAggregate;
use domain::{error::RepositoryError, repositories::commission_repository::CommissionRepository};
use sea_orm::*;

use crate::connection::connect;
use crate::helpers::join_parallel;

#[derive(Clone, Debug)]
pub struct SeaOrmCommissionRepository {
    db: DatabaseConnection,
}

pub struct SeaOrmCommissionTransportRepository {
    pub db: sea_orm::DatabaseConnection,
}

impl SeaOrmCommissionRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self { db })
    }
}

#[async_trait]
impl domain::repositories::commission_repository::CommissionAggregateRepository
    for SeaOrmCommissionRepository
{
    async fn create_with_relations(
        &self,
        data: CommissionAggregate,
    ) -> Result<i32, RepositoryError> {
        let query = Commission::insert(data.commission)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let commission_id = query.last_insert_id;

        CommissionReason::insert(data.reason)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        join_parallel(data.transports.into_iter().map(|mut transport| {
            let db = self.db.clone();

            async move {
                transport.set(
                    commission_transport::Column::CommissionId,
                    commission_id.into(),
                );
                CommissionTransport::insert(transport).exec(&db).await
            }
        }))
        .await;

        join_parallel(data.officials.into_iter().map(|mut official| {
            let db = self.db.clone();

            async move {
                official.set(
                    commission_official::Column::CommissionId,
                    commission_id.into(),
                );
                CommissionOfficial::insert(official).exec(&db).await
            }
        }))
        .await;

        Ok(commission_id)
    }

    async fn find_with_relations(&self, id: i32) -> Result<CommissionAggregate, RepositoryError> {
        let commission = Commission::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Commission with id {} not found", id))
            })?;

        let commission_reason = CommissionReason::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("CommissionReason with id {} not found", id))
            })?;

        let commission_official = CommissionOfficial::find()
            .filter(commission_official::Column::CommissionId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let commission_transport = CommissionTransport::find()
            .filter(commission_transport::Column::CommissionId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(CommissionAggregate {
            commission: commission.into_active_model(),
            reason: commission_reason.into_active_model(),
            officials: commission_official
                .into_iter()
                .map(|official| official.into_active_model())
                .collect(),
            transports: commission_transport
                .into_iter()
                .map(|transport| transport.into_active_model())
                .collect(),
        })
    }
}

#[async_trait]
impl CommissionRepository for SeaOrmCommissionRepository {
    async fn create(&self, data: commission::ActiveModel) -> Result<i32, RepositoryError> {
        let res = data
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.id)
    }

    async fn find(
        &self,
        search: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<domain::entities::commission::Model>, RepositoryError> {
        let mut query = commission::Entity::find();

        query = query.limit(limit.unwrap_or(50));
        query = query.offset(offset.unwrap_or(0));

        let result = query
            .order_by_desc(commission::Column::EntryAt)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<domain::entities::commission::Model, RepositoryError> {
        use domain::entities::commission;
        use sea_orm::EntityTrait;

        let result = commission::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        match result {
            Some(model) => Ok(model),
            None => Err(RepositoryError::Database(format!(
                "Commission with id {} not found",
                id
            ))),
        }
    }

    async fn update(
        &self,
        id: i32,
        commission: domain::entities::commission::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        let mut commission = commission;
        commission.id = Set(id);

        let res = commission
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.id)
    }
    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync,
    {
        let query = commission::Entity::find()
            .into_partial_model::<R>()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(query)
    }
}

pub struct SeaOrmCommissionReasonRepository {
    pub db: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl domain::repositories::commission_repository::CommissionReasonRepository
    for SeaOrmCommissionReasonRepository
{
    async fn create(
        &self,
        data: domain::entities::commission_reason::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        let res = commission_reason::Entity::insert(data)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.last_insert_id)
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<domain::entities::commission_reason::Model, RepositoryError> {
        let result = commission_reason::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        match result {
            Some(model) => Ok(model),
            None => Err(RepositoryError::Database(format!(
                "CommissionReason with id {} not found",
                id
            ))),
        }
    }

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<domain::entities::commission_reason::Model>, RepositoryError> {
        use domain::entities::commission_reason;
        use sea_orm::EntityTrait;

        let result = commission_reason::Entity::find()
            .filter(commission_reason::Column::CommissionId.eq(commission_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update(
        &self,
        id: i32,
        mut data: domain::entities::commission_reason::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        use sea_orm::ActiveModelTrait;
        data.id = sea_orm::Set(id);

        let res = data
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.id)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        use domain::entities::commission_reason;
        use sea_orm::ActiveModelTrait;
        use sea_orm::EntityTrait;

        let model = commission_reason::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(model) = model {
            let am: commission_reason::ActiveModel = model.into();
            am.delete(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
            Ok(())
        } else {
            Err(RepositoryError::Database(format!(
                "CommissionReason with id {} not found",
                id
            )))
        }
    }

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: Send + Sync,
    {
        Err(RepositoryError::Database("Not implemented".to_string()))
    }
}

pub struct SeaOrmCommissionOfficialRepository {
    pub db: sea_orm::DatabaseConnection,
}

#[async_trait::async_trait]
impl domain::repositories::commission_repository::CommissionOfficialRepository
    for SeaOrmCommissionOfficialRepository
{
    async fn create(
        &self,
        data: domain::entities::commission_official::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        let res = commission_official::Entity::insert(data)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.last_insert_id)
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<domain::entities::commission_official::Model, RepositoryError> {
        use domain::entities::commission_official;
        use sea_orm::EntityTrait;

        let result = commission_official::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        match result {
            Some(model) => Ok(model),
            None => Err(RepositoryError::Database(format!(
                "CommissionOfficial with id {} not found",
                id
            ))),
        }
    }

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<domain::entities::commission_official::Model>, RepositoryError> {
        use domain::entities::commission_official;
        use sea_orm::EntityTrait;

        let result = commission_official::Entity::find()
            .filter(commission_official::Column::CommissionId.eq(commission_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update(
        &self,
        id: i32,
        mut data: domain::entities::commission_official::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        use sea_orm::ActiveModelTrait;
        data.id = sea_orm::Set(id);

        let res = data
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.id)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        use domain::entities::commission_official;
        use sea_orm::ActiveModelTrait;
        use sea_orm::EntityTrait;

        let model = commission_official::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(model) = model {
            let am: domain::entities::commission_official::ActiveModel = model.into();
            am.delete(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
            Ok(())
        } else {
            Err(RepositoryError::Database(format!(
                "CommissionOfficial with id {} not found",
                id
            )))
        }
    }

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: Send + Sync,
    {
        Err(RepositoryError::Database("Not implemented".to_string()))
    }
}

#[async_trait::async_trait]
impl domain::repositories::commission_repository::CommissionTransportRepository
    for SeaOrmCommissionTransportRepository
{
    async fn create(
        &self,
        data: domain::entities::commission_transport::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        let res = commission_transport::Entity::insert(data)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.last_insert_id)
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<domain::entities::commission_transport::Model, RepositoryError> {
        let result = commission_transport::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        match result {
            Some(model) => Ok(model),
            None => Err(RepositoryError::Database(format!(
                "CommissionTransport with id {} not found",
                id
            ))),
        }
    }

    async fn find_by_commission_id(
        &self,
        commission_id: i32,
    ) -> Result<Vec<domain::entities::commission_transport::Model>, RepositoryError> {
        let result = commission_transport::Entity::find()
            .filter(commission_transport::Column::CommissionId.eq(commission_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result)
    }

    async fn update(
        &self,
        id: i32,
        mut data: domain::entities::commission_transport::ActiveModel,
    ) -> Result<i32, RepositoryError> {
        use sea_orm::ActiveModelTrait;
        data.id = sea_orm::Set(id);

        let res = data
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(res.id)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        use domain::entities::commission_transport;
        use sea_orm::ActiveModelTrait;
        use sea_orm::EntityTrait;

        let model = commission_transport::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(model) = model {
            let am: domain::entities::commission_transport::ActiveModel = model.into();
            am.delete(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
            Ok(())
        } else {
            Err(RepositoryError::Database(format!(
                "CommissionTransport with id {} not found",
                id
            )))
        }
    }

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: Send + Sync,
    {
        Err(RepositoryError::Database("Not implemented".to_string()))
    }
}
