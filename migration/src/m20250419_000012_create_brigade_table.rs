use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000012_create_brigade_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating brigade table");

        manager
            .create_table(
                Table::create()
                    .table(Brigade::Table)
                    .col(
                        ColumnDef::new(Brigade::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Brigade::Name)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Brigade::Table)
            .columns([Brigade::Name])
            .values_panic(["Brigada A".into()])
            .values_panic(["Brigada B".into()])
            .values_panic(["Brigada C".into()])
            .values_panic(["Brigada D".into()])
            .values_panic(["Brigada E".into()])
            .values_panic(["Brigada F".into()])
            .values_panic(["Brigada G".into()])
            .values_panic(["Brigada 1".into()])
            .values_panic(["Brigada 2".into()])
            .values_panic(["Brigada 3".into()])
            .values_panic(["Brigada 4".into()])
            .values_panic(["Brigada 5".into()])
            .values_panic(["Brigada 6".into()])
            .values_panic(["Brigada 7".into()])
            .values_panic(["Brigada 8".into()])
            .values_panic(["Brigada 9".into()])
            .values_panic(["Directivos".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Dropping brigade table");

        manager
            .drop_table(Table::drop().table(Brigade::Table).to_owned())
            .await?;

        println!("✅ Brigade table dropped");

        Ok(())
    }
}

#[derive(Iden)]
pub enum Brigade {
    Table,
    Id,
    Name,
}
