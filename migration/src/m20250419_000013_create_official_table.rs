use sea_orm_migration::prelude::*;

use crate::m20250618_113709_create_base_tables::Base;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000013_create_official_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating official table");

        manager
            .create_table(
                Table::create()
                    .table(Charge::Table)
                    .col(
                        ColumnDef::new(Charge::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Charge::Name).string_len(255).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Hierarchy::Table)
                    .col(
                        ColumnDef::new(Hierarchy::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Hierarchy::Name).string_len(255).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Official::Table)
                    .col(
                        ColumnDef::new(Official::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Official::Ci)
                            .string_len(20)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Official::LastName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Official::FirstName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Official::Phone).text().not_null())
                    .col(ColumnDef::new(Official::ChargeId).integer().not_null())
                    .col(ColumnDef::new(Official::HierarchyId).integer().not_null())
                    .col(ColumnDef::new(Official::BrigadeId).integer().not_null())
                    .col(ColumnDef::new(Official::Code).integer().null())
                    .col(ColumnDef::new(Official::BaseId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_official_base")
                            .from(Official::Table, Official::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_official_charge")
                            .from(Official::Table, Official::ChargeId)
                            .to(Charge::Table, Charge::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_official_hierarchy")
                            .from(Official::Table, Official::HierarchyId)
                            .to(Hierarchy::Table, Hierarchy::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_official_brigade")
                            .from(Official::Table, Official::BrigadeId)
                            .to(Brigade::Table, Brigade::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed common hierarchies
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Hierarchy::Table)
                    .columns([Hierarchy::Id, Hierarchy::Name])
                    .values_panic([1.into(), "Oficial".into()])
                    .values_panic([2.into(), "Primer Oficial".into()])
                    .values_panic([3.into(), "Oficial Jefe".into()])
                    .values_panic([4.into(), "Inspector".into()])
                    .values_panic([5.into(), "Primer Inspector".into()])
                    .values_panic([6.into(), "Inspector Jefe".into()])
                    .values_panic([7.into(), "Comisario".into()])
                    .values_panic([8.into(), "Primer Comisario".into()])
                    .values_panic([9.into(), "Comisario Jefe".into()])
                    .values_panic([10.into(), "Comisario General".into()])
                    .values_panic([11.into(), "Comisario Mayor".into()])
                    .values_panic([12.into(), "Comisario Superior".into()])
                    .to_owned(),
            )
            .await?;

        // Seed common charges
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Charge::Table)
                    .columns([Charge::Id, Charge::Name])
                    .values_panic([1.into(), "Funcionario".into()])
                    .values_panic([2.into(), "Adjunto de Brigada".into()])
                    .values_panic([3.into(), "Jefe de Brigada".into()])
                    .values_panic([4.into(), "Jefe de Operaciones".into()])
                    .values_panic([5.into(), "Adjunto a Operaciones".into()])
                    .values_panic([6.into(), "Subdirector".into()])
                    .values_panic([7.into(), "Director".into()])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Dropping official table");

        manager
            .drop_table(Table::drop().table(Official::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Charge::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Hierarchy::Table).cascade().to_owned())
            .await?;

        println!("✅ Official table dropped");

        Ok(())
    }
}

#[derive(Iden)]
pub enum Official {
    Table,
    Id,
    Ci,
    LastName,
    FirstName,
    BaseId,
    Phone,
    ChargeId,
    HierarchyId,
    BrigadeId,
    Code,
}

#[derive(Iden)]
pub enum Charge {
    Table,
    Name,
    Id,
}

#[derive(Iden)]
pub enum Hierarchy {
    Table,
    Name,
    Id,
}

#[derive(Iden)]
pub enum Brigade {
    Table,
    Id,
}
