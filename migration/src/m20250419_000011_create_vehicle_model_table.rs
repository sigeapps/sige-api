use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000011_create_vehicle_model_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VehicleModel::Table)
                    .col(
                        ColumnDef::new(VehicleModel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(VehicleModel::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(VehicleModel::Brand).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vehicle_model_brand")
                            .from(VehicleModel::Table, VehicleModel::Brand)
                            .to(Brand::Table, Brand::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VehicleModel::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum VehicleModel {
    #[iden = "vehicle_model"]
    Table,
    Id,
    Name,
    Brand,
}

#[derive(Iden)]
pub enum Brand {
    Table,
    Id,
}
