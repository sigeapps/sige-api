use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250504_000015_create_commissions_table::Transport, m20250618_113709_create_base_tables::Base,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TransportIssuance::Table)
                    .if_not_exists()
                    .col(pk_auto(TransportIssuance::Id))
                    .col(integer(TransportIssuance::AssignedPersonaId))
                    .col(integer(TransportIssuance::AssignedTransportId))
                    .col(date_time(TransportIssuance::DateTime).default(Expr::current_timestamp()))
                    .col(string(TransportIssuance::Type))
                    .col(integer(TransportIssuance::AssignanceDays))
                    .col(integer(TransportIssuance::AuthById))
                    .col(integer(TransportIssuance::BaseId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transport-issuance-base_id")
                            .from(TransportIssuance::Table, TransportIssuance::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transport-issuance-persona_id")
                            .from(
                                TransportIssuance::Table,
                                TransportIssuance::AssignedPersonaId,
                            )
                            .to(Persona::Table, Persona::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transport-issuance-weapon_id")
                            .from(
                                TransportIssuance::Table,
                                TransportIssuance::AssignedTransportId,
                            )
                            .to(Transport::Table, Transport::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transport-issuance-auth_by_id")
                            .from(TransportIssuance::Table, TransportIssuance::AuthById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransportIssuanceReturn::Table)
                    .if_not_exists()
                    .col(pk_auto(TransportIssuanceReturn::Id))
                    .col(integer(TransportIssuanceReturn::IssuanceId).unique_key())
                    .col(
                        date_time(TransportIssuanceReturn::ReturnedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TransportIssuanceReturn::Observations)
                            .text()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transport-issuance_return-issuance_id")
                            .from(
                                TransportIssuanceReturn::Table,
                                TransportIssuanceReturn::IssuanceId,
                            )
                            .to(TransportIssuance::Table, TransportIssuance::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TransportIssuanceReturn::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(TransportIssuance::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Persona {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TransportIssuance {
    Table,
    Id,
    AssignedPersonaId,
    AssignedTransportId,
    DateTime,
    Type,
    AssignanceDays,
    AuthById,
    BaseId,
}

#[derive(DeriveIden)]
enum TransportIssuanceReturn {
    Table,
    Id,
    IssuanceId,
    ReturnedAt,
    Observations,
}
