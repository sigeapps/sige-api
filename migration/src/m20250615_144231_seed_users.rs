use sea_orm::*;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20250419_000004_create_role_permissions_table::RolePermission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Importar las tablas necesarias
        use crate::m20250419_000001_create_role_table::Role;
        use crate::m20250419_000002_create_user_table::User;

        println!("Creating additional roles");

        // Verificar si los roles ya existen antes de insertarlos
        let db = manager.get_connection();

        // El rol "Usuario" ya debería existir desde la migración de creación de roles
        // Solo insertar roles sudo y Admin

        let sudo_exists = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM role WHERE name = 'sudo'".to_string(),
            ))
            .await?;

        if sudo_exists.is_none() || sudo_exists.unwrap().try_get::<i64>("", "count")? == 0 {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(Role::Table)
                        .columns([Role::Name])
                        .values_panic(["sudo".into()])
                        .to_owned(),
                )
                .await?;
        }

        println!("Role sudo created");

        let admin_exists = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM role WHERE name = 'Admin'".to_string(),
            ))
            .await?;

        if admin_exists.is_none() || admin_exists.unwrap().try_get::<i64>("", "count")? == 0 {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(Role::Table)
                        .columns([Role::Name])
                        .values_panic(["Admin".into()])
                        .to_owned(),
                )
                .await?;
        }

        println!("Role admin created");

        // Obtener el ID del rol sudo
        let sudo_role_row = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT id FROM role WHERE name = 'sudo'".to_string(),
            ))
            .await?;

        let sudo_role_id = if let Some(row) = sudo_role_row {
            row.try_get::<i32>("", "id")?
        } else {
            return Err(DbErr::Custom(
                "No se pudo encontrar el rol sudo".to_string(),
            ));
        };

        // Obtener el ID del rol Admin
        let admin_role_row = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT id FROM role WHERE name = 'Admin'".to_string(),
            ))
            .await?;

        let admin_role_id = if let Some(row) = admin_role_row {
            row.try_get::<i32>("", "id")?
        } else {
            return Err(DbErr::Custom(
                "No se pudo encontrar el rol Admin".to_string(),
            ));
        };

        // Verificar si los usuarios ya existen antes de insertarlos
        let sigar_exists = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM \"user\" WHERE name = 'sigar'".to_string(),
            ))
            .await?;

        if sigar_exists.is_none() || sigar_exists.unwrap().try_get::<i64>("", "count")? == 0 {
            // Insertar usuario sigar con el ID real del rol sudo
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(User::Table)
                        .columns([User::Name, User::PasswordHash, User::RoleId, User::PersonaId])
                        .values_panic([
                            "sigar".into(),
                            "$argon2id$v=19$m=19456,t=2,p=1$iFyKwovvE6HrcG5KmoE+Gw$KVjkvuGGoN1J+vRxFFDGLIBZOWc45lnDjFFTMG4NKTY".into(),
                            sudo_role_id.into(),
                            1.into()
                        ])
                        .to_owned(),
                )
                .await?;

            println!("User sigar created");
        }

        let admin_user_exists = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM \"user\" WHERE name = 'admin'".to_string(),
            ))
            .await?;

        if admin_user_exists.is_none()
            || admin_user_exists.unwrap().try_get::<i64>("", "count")? == 0
        {
            // Insertar usuario admin con el ID real del rol Admin
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(User::Table)
                        .columns([User::Name, User::PasswordHash, User::RoleId])
                        .values_panic([
                            "admin".into(),
                            "$argon2id$v=19$m=19456,t=2,p=1$JmUr+NpAn7nc0+tta9LjeQ$CzKQExOVudjmx33T0hPpUndc6FsQCTPWdfsxiGTQYeU".into(),
                            admin_role_id.into(),
                        ])
                        .to_owned(),
                )
                .await?;

            println!("User admin created");
        }

        // Asignar todos los permisos al rol sudo usando el ID real
        use domain::auth::permissions::Permission as DomainPermission;

        for permission in DomainPermission::iter() {
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(RolePermission::Table)
                        .columns([RolePermission::RoleId, RolePermission::PermissionId])
                        .values_panic([sudo_role_id.into(), permission.to_value().into()])
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
