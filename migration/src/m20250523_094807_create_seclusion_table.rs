use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250504_000015_create_commissions_table::Commission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tables
        manager
            .create_table(
                Table::create()
                    .table(FamilyRelationship::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FamilyRelationship::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FamilyRelationship::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        println!("✅ Family Relationship table created");

        manager
            .create_table(
                Table::create()
                    .table(SeclusionStatuses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SeclusionStatuses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SeclusionStatuses::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        println!("✅ Seclusion Statuses table created");

        // Seed family relationships
        let relationships = ["Padre", "Madre", "Hermano/a", "Tío/a", "Abuelo/a", "Otro"];

        for relationship in relationships {
            let insert = Query::insert()
                .into_table(FamilyRelationship::Table)
                .columns([FamilyRelationship::Name])
                .values_panic([relationship.into()])
                .on_conflict(
                    OnConflict::column(FamilyRelationship::Name)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        println!("✅ Family Relationships seeded");

        // Seed seclusion statuses
        let statuses = ["Activo", "Liberado", "Trasladado"];

        for status in statuses {
            let insert = Query::insert()
                .into_table(SeclusionStatuses::Table)
                .columns([SeclusionStatuses::Name])
                .values_panic([status.into()])
                .on_conflict(
                    OnConflict::column(SeclusionStatuses::Name)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        println!("✅ Seclusion Statuses seeded");

        manager
            .create_table(
                Table::create()
                    .table(Seclusion::Table)
                    .if_not_exists()
                    .col(pk_auto(Seclusion::Id))
                    .col(string(Seclusion::Photo))
                    .col(string(Seclusion::Ci).char_len(9).not_null())
                    .col(date(Seclusion::Birthdate).not_null())
                    .col(integer(Seclusion::Age).not_null())
                    .col(string(Seclusion::LastName).char_len(255).not_null())
                    .col(string(Seclusion::FirstName).char_len(255).not_null())
                    .col(string(Seclusion::Reason).not_null())
                    .col(string(Seclusion::ExitReason))
                    .col(string(Seclusion::PhysicalState).not_null())
                    .col(string(Seclusion::Outfit))
                    .col(string(Seclusion::Belongings))
                    .col(string(Seclusion::Observations))
                    .col(date(Seclusion::ExitAt))
                    .col(
                        timestamp_with_time_zone(Seclusion::CreatedAt)
                            .default(Keyword::CurrentTimestamp)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TemporalSeclusion::Table)
                    .col(pk_auto(TemporalSeclusion::Id))
                    .col(integer(TemporalSeclusion::CommissionId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_temporal-seclusion_commission")
                            .from(TemporalSeclusion::Table, TemporalSeclusion::CommissionId)
                            .to(Commission::Table, Commission::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(TemporalSeclusion::Ci).char_len(9).not_null())
                    .col(string(TemporalSeclusion::LastName).char_len(255).not_null())
                    .col(
                        string(TemporalSeclusion::FirstName)
                            .char_len(255)
                            .not_null(),
                    )
                    .col(integer(TemporalSeclusion::StatusId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_temporal-seclusion_seclusion_status")
                            .from(TemporalSeclusion::Table, TemporalSeclusion::StatusId)
                            .to(SeclusionStatuses::Table, SeclusionStatuses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        timestamp_with_time_zone(TemporalSeclusion::CreatedAt)
                            .default(Keyword::CurrentTimestamp)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SeclusionVisit::Table)
                    .if_not_exists()
                    .col(pk_auto(SeclusionVisit::Id))
                    .col(integer(SeclusionVisit::SeclusionId).not_null())
                    .col(string(SeclusionVisit::Ci).char_len(9).not_null())
                    .col(string(SeclusionVisit::LastName).char_len(255).not_null())
                    .col(string(SeclusionVisit::FirstName).char_len(255).not_null())
                    .col(integer(SeclusionVisit::RelationshipId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_seclusion_visit_relationship")
                            .from(SeclusionVisit::Table, SeclusionVisit::RelationshipId)
                            .to(FamilyRelationship::Table, FamilyRelationship::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(SeclusionVisit::Phone).char_len(255).not_null())
                    .col(date(SeclusionVisit::Date).not_null())
                    .col(date(SeclusionVisit::Address).not_null())
                    .col(date(SeclusionVisit::Reason))
                    .col(
                        timestamp_with_time_zone(SeclusionVisit::CreatedAt)
                            .default(Keyword::CurrentTimestamp)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_seclusion-visit_seclusion")
                            .from(SeclusionVisit::Table, SeclusionVisit::SeclusionId)
                            .to(Seclusion::Table, Seclusion::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Seclusion::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(SeclusionVisit::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TemporalSeclusion::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .cascade()
                    .table(SeclusionStatuses::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .cascade()
                    .table(FamilyRelationship::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Seclusion {
    Table,
    Id,
    Photo,
    Ci,
    Birthdate,
    Age,
    LastName,
    FirstName,
    Reason,
    ExitReason,
    PhysicalState,
    Outfit,
    Belongings,
    Observations,
    ExitAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TemporalSeclusion {
    Table,
    Id,
    CommissionId,
    Ci,
    LastName,
    FirstName,
    StatusId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum SeclusionVisit {
    Table,
    Id,
    SeclusionId,
    Ci,
    LastName,
    FirstName,
    Phone,
    Date,
    Address,
    RelationshipId,
    Reason,
    CreatedAt,
}

#[derive(DeriveIden)]
enum FamilyRelationship {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]

enum SeclusionStatuses {
    Table,
    Id,
    Name,
}
