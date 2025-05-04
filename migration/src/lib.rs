pub use sea_orm_migration::prelude::*;

mod m20250419_000001_create_role_table;
mod m20250419_000002_create_user_table;
mod m20250419_000003_create_permission_table;
mod m20250419_000004_create_role_permissions_table;
mod m20250419_000005_create_organism_table;
mod m20250419_000006_create_division_table;
mod m20250419_000007_create_register_table;
mod m20250419_000008_create_state_table;
mod m20250419_000009_create_municipality_table;
mod m20250419_000010_create_brand_table;
mod m20250419_000011_create_vehicle_model_table;
mod m20250419_000012_create_brigade_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250419_000001_create_role_table::Migration),
            Box::new(m20250419_000002_create_user_table::Migration),
            Box::new(m20250419_000003_create_permission_table::Migration),
            Box::new(m20250419_000004_create_role_permissions_table::Migration),
            Box::new(m20250419_000005_create_organism_table::Migration),
            Box::new(m20250419_000006_create_division_table::Migration),
            Box::new(m20250419_000007_create_register_table::Migration),
            Box::new(m20250419_000008_create_state_table::Migration),
            Box::new(m20250419_000009_create_municipality_table::Migration),
            Box::new(m20250419_000010_create_brand_table::Migration),
            Box::new(m20250419_000011_create_vehicle_model_table::Migration),
            Box::new(m20250419_000012_create_brigade_table::Migration),
        ]
    }
}
