use sea_orm_migration::prelude::*;

use crate::m20250419_000001_create_role_table::Role;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000002_create_user_table.rs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::PersonaId)
                            .integer()
                            .null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-persona_id")
                            .from(User::Table, User::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::RoleId).integer().default(1).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-role_id")
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed initial test user
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(User::Table)
                    .columns([User::Name, User::PasswordHash])
                    .values_panic(["taller".into(), "$argon2id$v=19$m=19456,t=2,p=1$c6ge1SHEIvbV5agUlnkHJQ$Z5SKu0Urq3f+I04odyHC1A7j3x9iuiriMMSuEeZK9PE".into()])
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    PersonaId,
    Name,
    PasswordHash,
    RoleId,
}

#[derive(Iden)]
pub enum Persona {
    Table,
    Id,
}
