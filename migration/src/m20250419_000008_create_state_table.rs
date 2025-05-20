use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000008_create_state_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(State::Table)
                    .col(
                        ColumnDef::new(State::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(State::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        println!("🔨 Seeding states");

        let states = [
            ("Amazonas", 1),
            ("Anzoátegui", 2),
            ("Apure", 3),
            ("Aragua", 4),
            ("Barinas", 5),
            ("Bolívar", 6),
            ("Carabobo", 7),
            ("Cojedes", 8),
            ("Delta Amacuro", 9),
            ("Falcón", 10),
            ("Guárico", 11),
            ("Lara", 12),
            ("Mérida", 13),
            ("Miranda", 14),
            ("Monagas", 15),
            ("Nueva Esparta", 16),
            ("Portuguesa", 17),
            ("Sucre", 18),
            ("Táchira", 19),
            ("Trujillo", 20),
            ("Vargas", 21),
            ("Yaracuy", 22),
            ("Zulia", 23),
            ("Distrito Capital", 24),
        ];

        for (name, id) in states {
            let insert = Query::insert()
                .into_table(State::Table)
                .columns([State::Id, State::Name])
                .values_panic([id.into(), name.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        println!("✅ States seeded");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum State {
    Table,
    Id,
    Name,
}
