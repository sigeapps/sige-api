use async_trait::async_trait;
use chrono::NaiveDate;
use domain::entities::{prelude::*, *};
use domain::error::RepositoryError;
use domain::repositories::register_repository::RegisterRepository;
use sea_orm::*;

use crate::connection::connect;

#[derive(Clone, Debug)]
pub struct SeaOrmRegisterRepository {
    db: DatabaseConnection,
}

impl SeaOrmRegisterRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self { db })
    }
}

#[async_trait]
impl RegisterRepository for SeaOrmRegisterRepository {
    async fn create(&self, register: register::ActiveModel) -> Result<(), RepositoryError> {
        Register::insert(register)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<register::Model>, RepositoryError> {
        let register = Register::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(register)
    }

    async fn find(
        &self,
        search: Option<String>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<register::Model>, RepositoryError> {
        let mut query = Register::find();

        if let Some(from_date) = from_date {
            query = query.filter(register::Column::EntryDate.gte(from_date));
        }

        if let Some(to_date) = to_date {
            query = query.filter(register::Column::EntryDate.lte(to_date));
        }

        if let Some(search) = search {
            query = query.filter(
                Condition::all()
                    .add(register::Column::LastName.contains(&search))
                    .add(register::Column::FirstName.contains(&search)),
            );
        }

        if let Some(limit) = limit {
            query = query.limit(limit as u64);
        }

        if let Some(offset) = offset {
            query = query.offset(offset as u64);
        }

        let registers = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(registers)
    }

    async fn find_partial<R>(&self) -> Result<Vec<R>, RepositoryError>
    where
        R: PartialModelTrait + FromQueryResult + Send + Sync,
    {
        let query = Register::find()
            .into_partial_model::<R>()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(query)
    }

    async fn update(&self, register: register::ActiveModel) -> Result<(), RepositoryError> {
        Register::update(register)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        Register::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
