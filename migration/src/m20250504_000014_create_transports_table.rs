use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250504_000014_create_transports_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("📦 Creating transport lookup tables (Brands, Types, Units)...");

        // Create transport_types lookup table
        manager
            .create_table(
                Table::create()
                    .table(TransportType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransportType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TransportType::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransportStatuses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransportStatuses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TransportStatuses::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create transports table - Must come after brands, models, transport types/units, and the ENUM
        println!("🏗️ Creating transports table...");
        manager
            .create_table(
                Table::create()
                    .table(Transport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transport::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transport::TypeId).integer().not_null())
                    .col(ColumnDef::new(Transport::Details).string().null())
                    .col(ColumnDef::new(Transport::BrandId).integer().not_null())
                    .col(ColumnDef::new(Transport::ModelId).integer().not_null())
                    .col(
                        ColumnDef::new(Transport::Plate)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Transport::SerialPhoto).string().null())
                    .col(ColumnDef::new(Transport::Unit).string_len(50).not_null())
                    .col(ColumnDef::new(Transport::StatusId).integer().null()) // Nullable status
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transports-status_id")
                            .from(Transport::Table, Transport::StatusId)
                            .to(TransportStatuses::Table, TransportStatuses::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transports-type_id")
                            .from(Transport::Table, Transport::TypeId)
                            .to(TransportType::Table, TransportType::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transports-brand_id")
                            .from(Transport::Table, Transport::BrandId)
                            .to(Brand::Table, Brand::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transports-model_id")
                            .from(Transport::Table, Transport::ModelId)
                            .to(VehicleModel::Table, VehicleModel::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    // Note: Status ENUM does not need a foreign key constraint
                    .col(
                        ColumnDef::new(Transport::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Keyword::CurrentTimestamp),
                    )
                    .to_owned(),
            )
            .await?;
        println!("✅ Transports table created");

        println!("🔨 Seeding transport statuses");

        let statuses = ["Verificado", "Solicitado", "Espera por experto", "Entregado"];

        for status in statuses {
            let insert = Query::insert()
                .into_table(TransportStatuses::Table)
                .columns([TransportStatuses::Name])
                .values_panic([status.into()])
                .on_conflict(
                    OnConflict::column(TransportStatuses::Name)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        println!("✅ Transport statuses seeded");

        println!("🔨 Seeding transport types");

        let types = ["Moto", "Carro", "Particular"];

        for transport_type in types {
            let insert = Query::insert()
                .into_table(TransportType::Table)
                .columns([TransportType::Name])
                .values_panic([transport_type.into()])
                .on_conflict(
                    OnConflict::column(TransportType::Name)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        println!("✅ Transport types seeded");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation to respect foreign keys

        println!("💥 Dropping transports table...");
        manager
            .drop_table(
                Table::drop()
                    .table(Transport::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        // Drop transport lookup tables
        manager
            .drop_table(
                Table::drop()
                    .table(TransportType::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        println!("✅ Transports table dropped");

        Ok(())
    }
}

// --- SeaORM Entity Names (for use in the migration) ---
// These Iden enums are used by the migration builder to refer to table and column names.
// You should define these in your SeaORM entity module(s) and potentially import them.
// For this migration file to compile, we define them locally.

#[derive(Iden)]
enum Brand {
    Table,
    Id,
}

#[derive(Iden)]
enum TransportType {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum TransportStatuses {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum VehicleModel {
    Table,
    Id,
}

#[derive(Iden)]
enum Transport {
    Table,
    Id,
    TypeId,
    Details,
    BrandId,
    ModelId,
    Plate,
    Unit,
    SerialPhoto,
    StatusId,
    CreatedAt,
}
