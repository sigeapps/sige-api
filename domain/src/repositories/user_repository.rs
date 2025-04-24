use crate::{entities::user, error::RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_username(
        &self,
        username: String,
    ) -> Result<Option<user::Model>, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<user::Model>, RepositoryError>;
    async fn create(&self, user: user::ActiveModel) -> Result<(), RepositoryError>;
}
