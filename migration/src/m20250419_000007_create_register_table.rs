use sea_orm_migration::prelude::*;

use crate::{
    m20250419_000005_create_organism_table::Organism,
    m20250419_000006_create_division_table::Division, m20250618_113709_create_base_tables::Base,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000007_create_register_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating register table");

        manager
            .create_table(
                Table::create()
                    .table(Register::Table)
                    .col(
                        ColumnDef::new(Register::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Register::Photo).string().null())
                    .col(ColumnDef::new(Register::Ci).string().not_null())
                    .col(ColumnDef::new(Register::LastName).string().not_null())
                    .col(ColumnDef::new(Register::FirstName).string().not_null())
                    .col(ColumnDef::new(Register::Organism).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-register-organism")
                            .from(Register::Table, Register::Organism)
                            .to(Organism::Table, Organism::Id),
                    )
                    .col(ColumnDef::new(Register::Division).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-register-division")
                            .from(Register::Table, Register::Division)
                            .to(Division::Table, Division::Id),
                    )
                    .col(
                        ColumnDef::new(Register::IsOfficial)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Register::EntryDate)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Register::ExitDate).timestamp().null())
                    .col(ColumnDef::new(Register::VisitReason).string().not_null())
                    .col(ColumnDef::new(Register::Observations).string().null())
                    .col(ColumnDef::new(Register::BaseId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-register-base")
                            .from(Register::Table, Register::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Register::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Register {
    Table,
    Id,
    Photo,
    Ci,
    LastName,
    FirstName,
    Organism,
    Division,
    IsOfficial,
    EntryDate,
    ExitDate,
    VisitReason,
    Observations,
    BaseId,
}
