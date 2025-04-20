pub use sea_orm_migration::prelude::*;

mod m20250419_000001_create_role_table;
mod m20250419_000002_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250419_000001_create_role_table::Migration),
            Box::new(m20250419_000002_create_user_table::Migration),
        ]
    }
}
