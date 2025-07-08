use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatusCondition::Table)
                    .if_not_exists()
                    .col(pk_auto(StatusCondition::Id))
                    .col(string(StatusCondition::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Band::Table)
                    .if_not_exists()
                    .col(pk_auto(Band::Id))
                    .col(string(Band::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Institution::Table)
                    .if_not_exists()
                    .col(pk_auto(Institution::Id))
                    .col(string(Institution::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Profession::Table)
                    .if_not_exists()
                    .col(pk_auto(Profession::Id))
                    .col(string(Profession::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DocumentType::Table)
                    .if_not_exists()
                    .col(pk_auto(DocumentType::Id))
                    .col(string(DocumentType::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        let statuses = ["Activo", "Vacaciones", "Reposo", "Civil"];

        for status in statuses {
            let insert = Query::insert()
                .into_table(StatusCondition::Table)
                .columns([StatusCondition::Name])
                .values_panic([status.into()])
                .on_conflict(
                    OnConflict::column(StatusCondition::Name)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StatusCondition::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StatusCondition {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Band {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Institution {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Profession {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum DocumentType {
    Table,
    Id,
    Name,
}
