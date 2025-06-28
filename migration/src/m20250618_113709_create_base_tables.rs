use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Base::Table)
                    .if_not_exists()
                    .col(pk_auto(Base::Id))
                    .col(string(Base::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Base::Table).cascade().to_owned())
            .await?;

        println!("Base table dropped");

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Base {
    Table,
    Id,
    Name,
}
