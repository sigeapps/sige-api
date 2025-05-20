use crate::{entities::transport, error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait TransportRepository {
    async fn find(&self, search: Option<String>) -> Result<Vec<transport::Model>, RepositoryError>;
    async fn create(&self, transport: transport::ActiveModel) -> Result<(), RepositoryError>;
}
