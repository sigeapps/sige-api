use std::sync::Arc;

use domain::{
    auth::permissions::Permission,
    entities::{permission, role, role_permission, user},
};
use password_auth::generate_hash;
use sea_orm::*;

use crate::dtos::user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO, GetUserDTO};

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

    pub async fn create_role(&self, role: CreateRoleDTO) -> Result<(), DbErr> {
        let active_role = role::ActiveModel {
            name: Set(role.name),
            ..Default::default()
        };

        let id = active_role.insert(&*self.db).await?.id;

        for permission in role.permissions {
            let active_permission = role_permission::ActiveModel {
                role_id: Set(id),
                permission_id: Set(permission),
                ..Default::default()
            };

            active_permission.insert(&*self.db).await?;
        }

        Ok(())
    }

    pub async fn find_roles(&self) -> Result<Vec<GetRoleDTO>, DbErr> {
        role::Entity::find()
            .into_partial_model::<GetRoleDTO>()
            .all(&*self.db)
            .await
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
            .column(permission::Column::Id)
            .into_tuple::<Permission>()
            .all(&*self.db)
            .await?;

        Ok(permissions)
    }

    pub async fn create(&self, mut user: CreateUserDTO) -> Result<i32, DbErr> {
        user.password_hash = generate_hash(user.password_hash);

        let id = user.into_active_model().insert(&*self.db).await?.id;

        Ok(id)
    }
}
