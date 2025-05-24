use std::sync::Arc;

use domain::entities::user;
use password_auth::generate_hash;
use sea_orm::*;

use crate::dtos::user::{CreateUserDTO, GetUserDTO};

#[derive(Debug, Clone)]
pub struct UserService {
    db: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        UserService { db }
    }

    pub async fn find_by_username(&self, username: String) -> Result<Option<GetUserDTO>, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Name.eq(username))
            .into_partial_model::<GetUserDTO>()
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<GetUserDTO>, DbErr> {
        let user = user::Entity::find_by_id(id)
            .into_partial_model::<GetUserDTO>()
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    pub async fn create(&self, mut user: CreateUserDTO) -> Result<(), DbErr> {
        user.password_hash = generate_hash(user.password_hash);

        let user = user.into_active_model();

        user::Entity::insert(user).exec(&*self.db).await?;

        Ok(())
    }
}
