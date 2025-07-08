use std::sync::Arc;

use domain::{
    auth::permissions::Permission,
    entities::{permission, persona, persona_situation, role, role_permission, user},
};
use password_auth::generate_hash;
use sea_orm::*;

use crate::dtos::{
    user::{CreateRoleDTO, CreateUserDTO, GetRoleDTO, GetUserDTO, UpdateUserDTO},
    CommonQueryFilterDTO,
};

#[derive(Debug, Clone)]
pub struct UserService {
    db: Arc<DatabaseConnection>,
}

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        UserService { db }
    }

    pub async fn find(&self, params: CommonQueryFilterDTO) -> Result<Vec<GetUserDTO>, DbErr> {
        let mut query = persona::Entity::find();

        query = query
            .left_join(user::Entity)
            .join(JoinType::LeftJoin, user::Relation::Role.def())
            .left_join(persona_situation::Entity)
            .join(JoinType::LeftJoin, persona_situation::Relation::Base2.def());

        if let Some(search) = params.search {
            query = query.filter(user::Column::Name.contains(search));
        }

        query
            .into_partial_model::<GetUserDTO>()
            .all(&*self.db)
            .await
    }

    pub async fn update_user(&self, mut dto: UpdateUserDTO, id: i32) -> Result<i32, DbErr> {
        dto.id = id;

        dto.password_hash = generate_hash(dto.password_hash);

        Ok(dto.into_active_model().update(&*self.db).await?.id)
    }

    pub async fn find_by_username(&self, username: String) -> Result<Option<GetUserDTO>, DbErr> {
        /*
        Buscamos usando la relación de persona, por que en seaORM no se puede hacer un join de 4 tablas,
        por lo que es más fácil encontrar la base del usuario haciendo el select en persona
        */
        let user = persona::Entity::find()
            .left_join(user::Entity)
            .join(JoinType::LeftJoin, user::Relation::Role.def())
            .left_join(persona_situation::Entity)
            .join(JoinType::LeftJoin, persona_situation::Relation::Base2.def())
            .join(
                JoinType::LeftJoin,
                persona_situation::Relation::Hierarchy2.def(),
            )
            .filter(Condition::any().add(user::Column::Name.eq(username)))
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
