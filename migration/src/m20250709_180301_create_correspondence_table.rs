use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250607_152359_create_lookup_tables::DocumentType, m20250618_113709_create_base_tables::Base,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Correspondence::Table)
                    .if_not_exists()
                    .col(pk_auto(Correspondence::Id))
                    .col(string(Correspondence::Type))
                    .col(integer(Correspondence::BaseId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_correspondence_base_id")
                            .from(Correspondence::Table, Correspondence::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .col(date_time(Correspondence::DateTime))
                    .col(date_time(Correspondence::ProcessDateTime))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CorrespondenceDocument::Table)
                    .if_not_exists()
                    .col(pk_auto(CorrespondenceDocument::Id))
                    .col(integer(CorrespondenceDocument::CorrespondenceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_correspondence_document_id")
                            .from(
                                CorrespondenceDocument::Table,
                                CorrespondenceDocument::CorrespondenceId,
                            )
                            .to(Correspondence::Table, Correspondence::Id),
                    )
                    .col(integer(CorrespondenceDocument::TypeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_correspondence_document_type")
                            .from(
                                CorrespondenceDocument::Table,
                                CorrespondenceDocument::TypeId,
                            )
                            .to(DocumentType::Table, DocumentType::Id),
                    )
                    .col(string(CorrespondenceDocument::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Correspondence::Table)
                    .cascade()
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(CorrespondenceDocument::Table)
                    .cascade()
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Correspondence {
    Table,
    Id,
    Type,
    BaseId,
    DateTime,
    ProcessDateTime,
}

#[derive(DeriveIden)]
enum CorrespondenceDocument {
    Table,
    Id,
    CorrespondenceId,
    TypeId,
    Name,
}
