use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000006_create_organism_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organism::Table)
                    .col(
                        ColumnDef::new(Organism::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Organism::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Organism::Phone).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Parish::Table)
                    .col(
                        ColumnDef::new(Parish::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Parish::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Organism::Table)
                    .columns([Organism::Name, Organism::Phone])
                    .values_panic(["CPNB".into(), "Cuerpo de Policia Nacional Bolivariana".into()])
                    .values_panic(["CICPC".into(), "Cuerpo de Investigacion Cientifica, Penal y Criminal".into()])
                    .values_panic(["SEBIN".into(), "Servicio Bolivariano de inteligencia".into()])
                    .values_panic(["DGCIM".into(), "Dirección General de Contrainteligencia Militar".into()])
                    .values_panic(["GOES".into(), "Grupo de Operaciones Estrategicas".into()])
                    .values_panic(["GNB".into(), "Guardia Nacional Bolivariana".into()])
                    .values_panic(["FANB".into(), "Fuerza Armada Nacional Bolivariana".into()])
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
                    .table(Organism::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .cascade()
                    .table(Parish::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Organism {
    Table,
    Id,
    Name,
    Phone,
}

#[derive(Iden)]
pub enum Parish {
    Table,
    Id,
    Name,
}
