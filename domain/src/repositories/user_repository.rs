use crate::entities::user;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_username(username: String) -> Option<user::Model>;
}
