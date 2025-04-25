use async_trait::async_trait;

use crate::{entities::register, error::RepositoryError};

#[async_trait]
pub trait RegisterRepository {
    async fn create(&self, register: register::ActiveModel) -> Result<(), RepositoryError>;

    async fn find_by_id(&self, id: i32) -> Result<Option<register::Model>, RepositoryError>;

    async fn find_all(&self) -> Result<Vec<register::Model>, RepositoryError>;

    async fn update(&self, register: register::ActiveModel) -> Result<(), RepositoryError>;

    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
