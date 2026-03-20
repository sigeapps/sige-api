use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{Iterable, Statement, ActiveEnum};
use domain::auth::permissions::Permission as DomainPermission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        
        for permission in DomainPermission::iter() {
            let id = permission.to_value();
            let name = format!("Permiso para {}", id);
            
            let insert_perm_sql = format!(
                "INSERT INTO permission (id, name) VALUES ('{}', '{}') ON CONFLICT (id) DO NOTHING",
                id, name
            );
            
            db.execute(Statement::from_string(manager.get_database_backend(), insert_perm_sql)).await?;

            let assigned_sudo_sql = format!(
                "INSERT INTO role_permission (role_id, permission_id) 
                 SELECT id, '{}' FROM role WHERE name IN ('sudo', 'Admin') 
                 ON CONFLICT (role_id, permission_id) DO NOTHING",
                id
            );
            
            db.execute(Statement::from_string(manager.get_database_backend(), assigned_sudo_sql)).await?;
        }
        
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
