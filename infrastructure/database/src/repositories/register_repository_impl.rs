use async_trait::async_trait;
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

    async fn find_all(&self) -> Result<Vec<register::Model>, RepositoryError> {
        Register::find()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
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
