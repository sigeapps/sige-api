use async_trait::async_trait;
use domain::entities::{prelude::*, *};
use domain::error::RepositoryError;
use domain::repositories::user_repository::UserRepository;
use sea_orm::*;
use tracing::info;

#[derive(Clone, Debug)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let db = Database::connect(database_url)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        info!("Database connected");

        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_username(
        &self,
        username: String,
    ) -> Result<Option<user::Model>, RepositoryError> {
        User::find()
            .filter(user::Column::Name.eq(username))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    async fn find_by_id(&self, user_id: i32) -> Result<Option<user::Model>, RepositoryError> {
        User::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    async fn create(&self, user: user::ActiveModel) -> Result<(), RepositoryError> {
        user::ActiveModel::from(user)
            .insert(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
}
