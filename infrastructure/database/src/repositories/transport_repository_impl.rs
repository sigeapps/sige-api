use async_trait::async_trait;
use domain::{
    entities::{prelude::*, *},
    error::RepositoryError,
    repositories::transport_repository::TransportRepository,
};
use sea_orm::*;

use crate::connection;

#[derive(Debug, Clone)]
pub struct SeaOrmTransportRepository {
    pub db: DatabaseConnection,
}

impl SeaOrmTransportRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = connection::connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(SeaOrmTransportRepository { db })
    }
}

#[async_trait]
impl TransportRepository for SeaOrmTransportRepository {
    async fn find(&self, search: Option<String>) -> Result<Vec<transport::Model>, RepositoryError> {
        let query = Transport::find().filter(if let Some(search) = search {
            Condition::all()
                .add(transport::Column::Details.contains(&search))
                .add(transport::Column::Plate.contains(&search))
                .add(transport::Column::Unit.contains(&search))
        } else {
            Condition::all()
        });

        let transports = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(transports)
    }

    async fn create(&self, transport: transport::ActiveModel) -> Result<(), RepositoryError> {
        Transport::insert(transport)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
