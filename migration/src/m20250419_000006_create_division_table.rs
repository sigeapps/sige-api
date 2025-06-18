use sea_orm_migration::prelude::*;

use crate::m20250419_000008_create_state_table::State;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000007_create_division_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating division table");

        manager
            .create_table(
                Table::create()
                    .table(Division::Table)
                    .col(
                        ColumnDef::new(Division::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Division::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Division::State).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_division_state")
                            .from(Division::Table, Division::State)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Division::Table)
            .columns([Division::Name, Division::State])
            .values_panic(["División de Inteligencia Estratégica (D.I.E)".into(), 24.into()])
            .values_panic(["Division de Operaciones y Tácticas Especiales (D.O.T.E)".into(), 24.into()])
            .values_panic(["Division Contra Drogas (D.C.D)".into(), 24.into()])
            .values_panic(["Division Contra la Delincuencia Organizada (D.C.D.O)".into(), 24.into()])
            .values_panic(["Division Antiextorsión y Secuestro (D.C.S.E)".into(), 24.into()])
            .values_panic(["Division de Investigaciones Penal (D.I.P)".into(), 24.into()])
            .values_panic(["División Contra Hurto Y Robo de Vehiculo (C.H.R.V)".into(), 24.into()])
            .values_panic(["División Canina (D-K9)".into(), 24.into()])
            .values_panic(["División Motorizada (D.M)".into(), 24.into()])
            .values_panic(["División de Patrullaje Vehicular (D.P.V)".into(), 24.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        println!("✅ Division table created");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Dropping division table");

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .cascade()
                    .table(Division::Table)
                    .to_owned(),
            )
            .await?;

        println!("✅ Division table dropped");

        Ok(())
    }
}

#[derive(Iden)]
pub enum Division {
    Table,
    Id,
    Name,
    State,
}
