use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000009_create_municipality_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Municipality::Table)
                    .col(
                        ColumnDef::new(Municipality::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Municipality::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Municipality::State).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_municipality_state")
                            .from(Municipality::Table, Municipality::State)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Municipality::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Municipality {
    Table,
    Id,
    Name,
    State,
}

#[derive(Iden)]
pub enum State {
    Table,
    Id,
}
