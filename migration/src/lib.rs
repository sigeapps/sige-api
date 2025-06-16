pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Database;

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
mod m20250419_000013_create_official_table;
mod m20250504_000014_create_transports_table;
mod m20250504_000015_create_commissions_table;
mod m20250523_094807_create_seclusion_table;
mod m20250602_011258_create_part_table;
mod m20250607_152359_create_lookup_tables;
mod m20250611_094810_create_persona_table;
mod m20250612_114832_create_country_tables;
mod m20250615_144231_seed_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250419_000008_create_state_table::Migration),
            Box::new(m20250419_000001_create_role_table::Migration),
            Box::new(m20250419_000002_create_user_table::Migration),
            Box::new(m20250419_000003_create_permission_table::Migration),
            Box::new(m20250419_000004_create_role_permissions_table::Migration),
            Box::new(m20250419_000005_create_organism_table::Migration),
            Box::new(m20250419_000006_create_division_table::Migration),
            Box::new(m20250419_000007_create_register_table::Migration),
            Box::new(m20250419_000009_create_municipality_table::Migration),
            Box::new(m20250419_000010_create_brand_table::Migration),
            Box::new(m20250419_000011_create_vehicle_model_table::Migration),
            Box::new(m20250419_000012_create_brigade_table::Migration),
            Box::new(m20250419_000013_create_official_table::Migration),
            Box::new(m20250504_000014_create_transports_table::Migration),
            Box::new(m20250504_000015_create_commissions_table::Migration),
            Box::new(m20250523_094807_create_seclusion_table::Migration),
            Box::new(m20250602_011258_create_part_table::Migration),
            Box::new(m20250607_152359_create_lookup_tables::Migration),
            Box::new(m20250611_094810_create_persona_table::Migration),
            Box::new(m20250612_114832_create_country_tables::Migration),
            Box::new(m20250615_144231_seed_users::Migration),
        ]
    }
}

#[tokio::main]
pub async fn migrate(db_url: &str) -> Result<(), DbErr> {
    let db = Database::connect(db_url).await?;

    Migrator::refresh(&db).await?;

    Ok(())
}
