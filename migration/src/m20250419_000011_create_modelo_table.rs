use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000011_create_modelo_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Modelo::Table)
                    .col(
                        ColumnDef::new(Modelo::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Modelo::Nombre)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Modelo::Marca)
                            .string_len(255)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_modelo_marca")
                            .from(Modelo::Table, Modelo::Marca)
                            .to(Marca::Table, Marca::Nombre)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Modelo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Modelo {
    Table,
    Id,
    Nombre,
    Marca,
}

#[derive(Iden)]
pub enum Marca {
    Table,
    Nombre,
}
