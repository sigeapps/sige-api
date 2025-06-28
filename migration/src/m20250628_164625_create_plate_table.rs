use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250419_000006_create_division_table::Division, m20250419_000008_create_state_table::State,
    m20250611_094810_create_persona_table::Persona, m20250618_113709_create_base_tables::Base,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Plate::Table)
                    .if_not_exists()
                    .col(pk_auto(Plate::Id))
                    .col(integer(Plate::DivisionId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate_division_id")
                            .from(Plate::Table, Plate::DivisionId)
                            .to(Division::Table, Division::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(date_time(Plate::Datetime).not_null())
                    .col(integer(Plate::BaseId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate_base_id")
                            .from(Plate::Table, Plate::DivisionId)
                            .to(Base::Table, Base::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Plate::StateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate_state_id")
                            .from(Plate::Table, Plate::DivisionId)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PlatePersona::Table)
                    .if_not_exists()
                    .col(pk_auto(PlatePersona::Id))
                    .col(integer(PlatePersona::PlateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate-persona_plate_id")
                            .from(PlatePersona::Table, PlatePersona::PlateId)
                            .to(Plate::Table, Plate::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PlatePersona::PersonaId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate-persona_persona_id")
                            .from(PlatePersona::Table, PlatePersona::PersonaId)
                            .to(Persona::Table, Persona::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PlatePersona::NewStateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate-persona_new_state_id")
                            .from(PlatePersona::Table, PlatePersona::NewStateId)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(PlatePersona::OldStateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_plate-persona_old_state_id")
                            .from(PlatePersona::Table, PlatePersona::OldStateId)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().cascade().table(Plate::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .cascade()
                    .table(PlatePersona::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Plate {
    Table,
    Id,
    DivisionId,
    Datetime,
    BaseId,
    StateId,
}

#[derive(DeriveIden)]
enum PlatePersona {
    Table,
    Id,
    PlateId,
    PersonaId,
    NewStateId,
    OldStateId,
}
