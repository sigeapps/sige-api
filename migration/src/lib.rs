pub use sea_orm_migration::prelude::*;

mod m20250419_000001_create_role_table;
mod m20250419_000002_create_user_table;
mod m20250419_000003_create_permission_table;
mod m20250419_000004_create_role_permissions_table;
mod m20250419_000005_create_organismo_table;
mod m20250419_000006_create_division_table;
mod m20250419_000007_create_register_table;
mod m20250419_000008_create_estado_table;
mod m20250419_000009_create_municipio_table;
mod m20250419_000010_create_marca_table;
mod m20250419_000011_create_modelo_table;
mod m20250419_000012_create_brigada_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250419_000001_create_role_table::Migration),
            Box::new(m20250419_000002_create_user_table::Migration),
            Box::new(m20250419_000003_create_permission_table::Migration),
            Box::new(m20250419_000004_create_role_permissions_table::Migration),
            Box::new(m20250419_000005_create_organismo_table::Migration),
            Box::new(m20250419_000006_create_division_table::Migration),
            Box::new(m20250419_000007_create_register_table::Migration),
            Box::new(m20250419_000008_create_estado_table::Migration),
            Box::new(m20250419_000009_create_municipio_table::Migration),
            Box::new(m20250419_000010_create_marca_table::Migration),
            Box::new(m20250419_000011_create_modelo_table::Migration),
            Box::new(m20250419_000012_create_brigada_table::Migration),
        ]
    }
}
