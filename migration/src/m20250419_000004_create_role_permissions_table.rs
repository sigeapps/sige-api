use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250419_000002_create_role_permissions_table.rs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .col(ColumnDef::new(RolePermission::RoleId).integer().not_null())
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(RolePermission::RoleId)
                            .col(RolePermission::PermissionId)
                            .name("pk_role_permissions"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_role_id")
                            .from(RolePermission::Table, RolePermission::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_permission_id")
                            .from(RolePermission::Table, RolePermission::PermissionId)
                            .to(Permission::Table, Permission::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RolePermission {
    Table,
    RoleId,
    PermissionId,
}

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Permission {
    Table,
    Id,
}
