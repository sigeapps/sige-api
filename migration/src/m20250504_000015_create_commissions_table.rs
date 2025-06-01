use sea_orm_migration::prelude::*;

use crate::{
    m20250419_000005_create_organism_table::Parish,
    m20250419_000009_create_municipality_table::Municipality,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250504_000015_create_commissions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Commission::Table)
                    .col(
                        ColumnDef::new(Commission::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Commission::BrigadeId).integer().not_null())
                    .col(
                        ColumnDef::new(Commission::AuthorizedOfficialId)
                            .integer()
                            .null(),
                    )
                    .col(ColumnDef::new(Commission::BossId).integer().null())
                    .col(
                        ColumnDef::new(Commission::EntryAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(), // Assuming entry_at should not be null with a default
                    )
                    .col(
                        ColumnDef::new(Commission::ExitAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Commission::StatusAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(Commission::Observations).text().null())
                    .col(
                        ColumnDef::new(Commission::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(), // Assuming created_at should not be null with a default
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commissions_brigade")
                            .from(Commission::Table, Commission::BrigadeId)
                            .to(Brigade::Table, Brigade::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commissions_official")
                            .from(Commission::Table, Commission::AuthorizedOfficialId)
                            .to(Official::Table, Official::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commissions_boss")
                            .from(Commission::Table, Commission::BossId)
                            .to(Official::Table, Official::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommissionOfficial::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommissionOfficial::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommissionOfficial::CommissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommissionOfficial::OfficialId)
                            .integer()
                            .not_null(),
                    )
                    // Add foreign keys
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_officials-commission_id")
                            .from(CommissionOfficial::Table, CommissionOfficial::CommissionId)
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_officials-official_id")
                            .from(CommissionOfficial::Table, CommissionOfficial::OfficialId)
                            .to(Official::Table, Official::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(
                        ColumnDef::new(CommissionOfficial::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Keyword::CurrentTimestamp),
                    ) // Use TIMESTAMPTZ
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommissionTransport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommissionTransport::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommissionTransport::CommissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommissionTransport::TransportId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_transports-commission_id")
                            .from(
                                CommissionTransport::Table,
                                CommissionTransport::CommissionId,
                            )
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_transports-transport_id")
                            .from(CommissionTransport::Table, CommissionTransport::TransportId)
                            .to(Transport::Table, Transport::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommissionSeizedTransport::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommissionSeizedTransport::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommissionSeizedTransport::CommissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommissionSeizedTransport::TransportId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_seized_transports-commission_id")
                            .from(
                                CommissionSeizedTransport::Table,
                                CommissionSeizedTransport::CommissionId,
                            )
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_seized_transports-transport_id")
                            .from(
                                CommissionSeizedTransport::Table,
                                CommissionSeizedTransport::TransportId,
                            )
                            .to(Transport::Table, Transport::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        let commission_tranport_index = Index::create()
            .name("uq-commission_transports-commission_transport")
            .table(CommissionTransport::Table)
            .col(CommissionTransport::CommissionId)
            .col(CommissionTransport::TransportId)
            .to_owned();

        let commission_seized_transport_index = Index::create()
            .name("uq-commission_seized_transports-commission_seized_transport")
            .table(CommissionSeizedTransport::Table)
            .col(CommissionSeizedTransport::CommissionId)
            .col(CommissionSeizedTransport::TransportId)
            .to_owned();

        let commission_official_index = Index::create()
            .name("uq-commission_officials-commission_official")
            .table(CommissionOfficial::Table)
            .col(CommissionOfficial::CommissionId)
            .col(CommissionOfficial::OfficialId)
            .to_owned();

        manager.create_index(commission_tranport_index).await?;
        manager.create_index(commission_official_index).await?;
        manager
            .create_index(commission_seized_transport_index)
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommissionReason::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommissionReason::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommissionReason::CommissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CommissionReason::Name).string().not_null()) // Reason name (text, or link to a reasons lookup)
                    .col(
                        ColumnDef::new(CommissionReason::Description)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CommissionReason::Zone).string()) // Zone (nullable text for now)
                    .col(
                        ColumnDef::new(CommissionReason::MunicipalityId)
                            .integer()
                            .not_null(),
                    ) // Link to municipality
                    .col(
                        ColumnDef::new(CommissionReason::ParishId)
                            .integer()
                            .not_null(),
                    ) // Link to municipality
                    // Add foreign keys
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_reasons-commission_id")
                            .from(CommissionReason::Table, CommissionReason::CommissionId)
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_reasons-municipality_id")
                            .from(CommissionReason::Table, CommissionReason::MunicipalityId)
                            .to(Municipality::Table, Municipality::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_reasons-parish_id")
                            .from(CommissionReason::Table, CommissionReason::ParishId)
                            .to(Parish::Table, Parish::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CommissionActualExit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommissionActualExit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommissionActualExit::CommissionId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    ) // Unique link to commission
                    .col(
                        ColumnDef::new(CommissionActualExit::ActualExitAt)
                            .timestamp_with_time_zone()
                            .default(Keyword::CurrentTimestamp),
                    ) // Use TIMESTAMPTZ
                    .col(ColumnDef::new(CommissionActualExit::Observations).text()) // Nullable text
                    // Add foreign key
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-commission_actual_exits-commission_id")
                            .from(
                                CommissionActualExit::Table,
                                CommissionActualExit::CommissionId,
                            )
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    ) // If commission deleted, remove its actual exit record
                    .col(
                        ColumnDef::new(CommissionActualExit::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Keyword::CurrentTimestamp),
                    ) // Use TIMESTAMPTZ
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop dependent tables first
        manager
            .drop_table(
                Table::drop()
                    .table(CommissionActualExit::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CommissionReason::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CommissionTransport::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CommissionOfficial::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        // Finally drop the main commission table
        manager
            .drop_table(Table::drop().table(Commission::Table).cascade().to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("uq-commission_transports-commission_transport")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("uq-commission_seized_transports-commission_seized_transport")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("uq-commission_officials-commission_official")
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Commission {
    Table,
    Id,
    BrigadeId,
    AuthorizedOfficialId,
    BossId,
    EntryAt,
    ExitAt,
    StatusAt,
    Observations,
    CreatedAt,
}

#[derive(Iden)]
enum CommissionReason {
    Table,
    Id,
    CommissionId, // FK to commissions
    Name,         // Reason name
    Description,
    Zone,
    MunicipalityId, // FK to municipalities
    ParishId,
}

#[derive(Iden)]
enum CommissionActualExit {
    Table,
    Id,
    CommissionId, // FK to commissions (unique)
    ActualExitAt,
    Observations,
    CreatedAt,
}

#[derive(Iden)]
enum CommissionOfficial {
    Table,
    Id,
    CommissionId, // FK to commissions
    OfficialId,   // FK to officials
    CreatedAt,
}

#[derive(Iden)]
enum CommissionTransport {
    Table,
    Id,
    CommissionId,
    TransportId,
}

#[derive(Iden)]
enum CommissionSeizedTransport {
    Table,
    Id,
    CommissionId,
    TransportId,
}

#[derive(Iden)]
pub enum Brigade {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Official {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Transport {
    Table,
    Id,
}
