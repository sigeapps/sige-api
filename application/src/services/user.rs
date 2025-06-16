use std::sync::Arc;

use domain::{
    auth::permissions::Permission,
    entities::{permission, role, role_permission, user},
};
use password_auth::generate_hash;
use sea_orm::*;

use crate::dtos::user::{CreateUserDTO, GetRoleDTO, GetUserDTO};

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
            .left_join(role::Entity)
            .into_partial_model::<GetUserDTO>()
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<GetUserDTO>, DbErr> {
        let user = user::Entity::find_by_id(id)
            .left_join(role::Entity)
            .into_partial_model::<GetUserDTO>()
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_permissions_by_role_id(
        &self,
        role_id: i32,
    ) -> Result<Vec<Permission>, DbErr> {
        let permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .left_join(permission::Entity)
            .select_only()
            .column(permission::Column::Name)
            .into_tuple::<Permission>()
            .all(&*self.db)
            .await?;

        Ok(permissions)
    }

    pub async fn create(&self, mut user: CreateUserDTO) -> Result<(), DbErr> {
        user.password_hash = generate_hash(user.password_hash);

        let user = user.into_active_model();

        user::Entity::insert(user).exec(&*self.db).await?;

        Ok(())
    }
}
