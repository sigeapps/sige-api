use crate::{entities::official, error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait OfficialRepository {
    // Should be able to search by name, ci, charge, hierarchy, code and brigade_id
    async fn find(
        &self,
        search: Option<String>,
        brigade_id: Option<i32>,
    ) -> Result<Vec<official::Model>, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<official::Model>, RepositoryError>;
    async fn create(&self, official: official::ActiveModel) -> Result<(), RepositoryError>;
    async fn update(&self, official: official::ActiveModel) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
