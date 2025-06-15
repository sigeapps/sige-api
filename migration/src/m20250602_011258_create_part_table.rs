use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250419_000013_create_official_table::Official;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("🔨 Creating parts table");

        manager
            .create_table(
                Table::create()
                    .table(Novelty::Table)
                    .if_not_exists()
                    .col(pk_auto(Novelty::Id))
                    .col(string(Novelty::Name))
                    .col(string(Novelty::Format))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Part::Table)
                    .if_not_exists()
                    .col(pk_auto(Part::Id))
                    .col(timestamp(Part::Date).default(Keyword::CurrentTimestamp))
                    .col(timestamp(Part::CreatedAt).default(Keyword::CurrentTimestamp))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PartOfficial::Table)
                    .if_not_exists()
                    .col(pk_auto(PartOfficial::Id))
                    .col(integer(PartOfficial::OfficialId))
                    .col(integer(PartOfficial::PartId))
                    .col(string(PartOfficial::Type))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-official-official_id")
                            .from(PartOfficial::Table, PartOfficial::OfficialId)
                            .to(Official::Table, Official::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-official-part_id")
                            .from(PartOfficial::Table, PartOfficial::PartId)
                            .to(Part::Table, Part::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PartDevelopment::Table)
                    .if_not_exists()
                    .col(pk_auto(PartDevelopment::Id))
                    .col(integer(PartDevelopment::PartId))
                    .col(string(PartDevelopment::Data))
                    .col(date_time(PartDevelopment::Date).default(Keyword::CurrentTimestamp))
                    .col(integer(PartDevelopment::TypeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-development-novelty_id")
                            .from(PartDevelopment::Table, PartDevelopment::TypeId)
                            .to(Novelty::Table, Novelty::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-development-part_id")
                            .from(PartDevelopment::Table, PartDevelopment::PartId)
                            .to(Part::Table, Part::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(PartDevelopment::CreatedAt).default(Keyword::CurrentTimestamp))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PartResponsability::Table)
                    .if_not_exists()
                    .col(pk_auto(PartResponsability::Id))
                    .col(integer(PartResponsability::PartId).not_null())
                    .col(string_len(PartResponsability::OfficialServiceCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialReceiveCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialBlockCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialWriterCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialDeliverCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialReceiveCustodyCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialHrCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialSupervisorCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialDirectorCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialDaeCi, 20).not_null())
                    .col(string_len(PartResponsability::OfficialDieCi, 20).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-part_id")
                            .from(PartResponsability::Table, PartResponsability::PartId)
                            .to(Part::Table, Part::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-service_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialServiceCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-receive_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialReceiveCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-block_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialBlockCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-writer_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialWriterCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-deliver_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialDeliverCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-receive_custody_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialReceiveCustodyCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-hr_ci")
                            .from(PartResponsability::Table, PartResponsability::OfficialHrCi)
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-supervisor_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialSupervisorCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-director_ci")
                            .from(
                                PartResponsability::Table,
                                PartResponsability::OfficialDirectorCi,
                            )
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-dae_ci")
                            .from(PartResponsability::Table, PartResponsability::OfficialDaeCi)
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-part-responsability-die_ci")
                            .from(PartResponsability::Table, PartResponsability::OfficialDieCi)
                            .to(Official::Table, Official::Ci)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        let part_official_index = Index::create()
            .name("uq-part_official-part_official")
            .table(PartOfficial::Table)
            .col(PartOfficial::PartId)
            .col(PartOfficial::OfficialId)
            .to_owned();

        manager.create_index(part_official_index).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Part::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PartDevelopment::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PartResponsability::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PartOfficial::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("uq-part_official-part_official")
                    .table(PartOfficial::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Part {
    Table,
    Id,
    Date,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PartDevelopment {
    Table,
    Id,
    PartId,
    Data,
    Date,
    // Reference to Novelty table
    TypeId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PartResponsability {
    Table,
    Id,
    PartId,
    OfficialServiceCi,
    OfficialReceiveCi,
    OfficialBlockCi,
    OfficialWriterCi,
    OfficialDeliverCi,
    OfficialReceiveCustodyCi,
    OfficialHrCi,
    OfficialSupervisorCi,
    OfficialDirectorCi,
    OfficialDaeCi,
    OfficialDieCi,
}

#[derive(DeriveIden)]
enum PartOfficial {
    Table,
    Id,
    PartId,
    OfficialId,
    // String cause will be controlled by frontend
    Type,
}

/// Lookup table for development types
///
#[derive(DeriveIden)]
enum Novelty {
    Table,
    Id,
    Name,
    Format,
}
