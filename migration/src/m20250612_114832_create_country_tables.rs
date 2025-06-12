use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CountryVerification::Table)
                    .if_not_exists()
                    .col(pk_auto(CountryVerification::Id))
                    .col(integer(CountryVerification::PersonaId).unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_country_verification_persona")
                            .from(CountryVerification::Table, CountryVerification::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(integer(CountryVerification::ConditionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_country_verification_condition")
                            .from(CountryVerification::Table, CountryVerification::PersonaId)
                            .to(StatusCondition::Table, StatusCondition::Id),
                    )
                    .col(boolean_null(CountryVerification::Reported))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .cascade()
                    .table(CountryVerification::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Persona {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum StatusCondition {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum CountryVerification {
    Table,
    Id,
    PersonaId,
    ConditionId,
    Reported,
}
