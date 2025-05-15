use crate::{entities::official, error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait OfficialRepository {
    async fn find(&self, search: Option<String>) -> Result<Vec<official::Model>, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<official::Model>, RepositoryError>;
    async fn find_by_brigade_id(
        &self,
        brigade_id: i32,
        search: Option<String>,
    ) -> Result<Vec<official::Model>, RepositoryError>;
    async fn create(&self, official: official::ActiveModel) -> Result<(), RepositoryError>;
    async fn update(&self, official: official::ActiveModel) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
