use sea_orm_migration::prelude::*;

use crate::m20250419_000004_create_role_permissions_table::RolePermission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Importar las tablas necesarias
        use crate::m20250419_000001_create_role_table::Role;
        use crate::m20250419_000002_create_user_table::User;

        // Insertar roles básicos si no existen
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Role::Table)
                    .columns([Role::Name])
                    .values_panic(["sudo".into()])
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Role::Table)
                    .columns([Role::Name])
                    .values_panic(["admin".into()])
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(User::Table)
                    .columns([User::Name, User::PasswordHash, User::RoleId])
                    .values_panic([
                        "sigar".into(),
                        "$argon2id$v=19$m=19456,t=2,p=1$iFyKwovvE6HrcG5KmoE+Gw$KVjkvuGGoN1J+vRxFFDGLIBZOWc45lnDjFFTMG4NKTY".into(),
                        2.into(),
                    ])
                    .to_owned(),
            )
            .await?;

        // Usuario moderador
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(User::Table)
                    .columns([User::Name, User::PasswordHash, User::RoleId])
                    .values_panic([
                        "admin".into(),
                        "$argon2id$v=19$m=19456,t=2,p=1$JmUr+NpAn7nc0+tta9LjeQ$CzKQExOVudjmx33T0hPpUndc6FsQCTPWdfsxiGTQYeU".into(),
                        3.into(),
                    ])
                    .to_owned(),
            )
            .await?;

        // Asignar todos los permisos al rol sudo (role_id = 2)
        use domain::auth::permissions::Permission as DomainPermission;

        for (index, _permission) in DomainPermission::all().iter().enumerate() {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(RolePermission::Table)
                        .columns([RolePermission::RoleId, RolePermission::PermissionId])
                        .values_panic([
                            2.into(), // sudo role
                            (index as i32 + 1).into(),
                        ])
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No se puede revertir la migración de usuarios
        Ok(())
    }
}
