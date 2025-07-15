use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WeaponType::Table)
                    .if_not_exists()
                    .col(pk_auto(WeaponType::Id))
                    .col(string(WeaponType::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WeaponBrand::Table)
                    .if_not_exists()
                    .col(pk_auto(WeaponBrand::Id))
                    .col(string(WeaponBrand::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WeaponModel::Table)
                    .if_not_exists()
                    .col(pk_auto(WeaponModel::Id))
                    .col(string(WeaponModel::Name).unique_key())
                    .col(integer(WeaponModel::BrandId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon_model-brand_id")
                            .from(WeaponModel::Table, WeaponModel::BrandId)
                            .to(WeaponBrand::Table, WeaponBrand::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Weapon::Table)
                    .if_not_exists()
                    .col(pk_auto(Weapon::Id))
                    .col(integer(Weapon::TypeId))
                    .col(integer(Weapon::ModelId))
                    .col(ColumnDef::new(Weapon::Photo).string().null())
                    .col(string(Weapon::Serial).unique_key())
                    .col(
                        timestamp_with_time_zone(Weapon::EntryAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Weapon::DocumentId).string().null())
                    .col(string(Weapon::Calibre))
                    .col(
                        ColumnDef::new(Weapon::ManteinanceAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(boolean(Weapon::HasCharger).default(false))
                    .col(ColumnDef::new(Weapon::Observations).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon-type_id")
                            .from(Weapon::Table, Weapon::TypeId)
                            .to(WeaponType::Table, WeaponType::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon-model_id")
                            .from(Weapon::Table, Weapon::ModelId)
                            .to(WeaponModel::Table, WeaponModel::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Issuance::Table)
                    .if_not_exists()
                    .col(pk_auto(Issuance::Id))
                    .col(integer(Issuance::AssignedPersonaId))
                    .col(integer(Issuance::AssignedWeaponId))
                    .col(boolean(Issuance::HasCharger).default(false))
                    .col(integer(Issuance::AmmoCount))
                    .col(
                        timestamp_with_time_zone(Issuance::DateTime)
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(Issuance::Reason))
                    .col(string(Issuance::Type))
                    .col(timestamp_with_time_zone(Issuance::AssignanceTime))
                    .col(integer(Issuance::AuthById))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance-persona_id")
                            .from(Issuance::Table, Issuance::AssignedPersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance-weapon_id")
                            .from(Issuance::Table, Issuance::AssignedWeaponId)
                            .to(Weapon::Table, Weapon::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance-auth_by_id")
                            .from(Issuance::Table, Issuance::AuthById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IssuanceReturn::Table)
                    .if_not_exists()
                    .col(pk_auto(IssuanceReturn::Id))
                    .col(integer(IssuanceReturn::IssuanceId))
                    .col(
                        timestamp_with_time_zone(IssuanceReturn::ReturnedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(boolean(IssuanceReturn::HasCharger).default(false))
                    .col(integer(IssuanceReturn::ReturnedAmmo))
                    .col(ColumnDef::new(IssuanceReturn::Observations).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance_return-issuance_id")
                            .from(IssuanceReturn::Table, IssuanceReturn::IssuanceId)
                            .to(Issuance::Table, Issuance::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IssuanceReturn::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Issuance::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Weapon::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WeaponModel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WeaponBrand::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(WeaponType::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Persona {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum WeaponType {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum WeaponBrand {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum WeaponModel {
    Table,
    Id,
    Name,
    BrandId,
}

#[derive(DeriveIden)]
enum Weapon {
    Table,
    Id,
    Photo,
    Serial,
    EntryAt,
    DocumentId,
    Calibre,
    ManteinanceAt,
    HasCharger,
    Observations,
    TypeId,
    ModelId,
}

#[derive(DeriveIden)]
enum Issuance {
    Table,
    Id,
    AssignedPersonaId,
    AssignedWeaponId,
    HasCharger,
    AmmoCount,
    DateTime,
    Reason,
    Type,
    AssignanceTime,
    AuthById,
}

#[derive(DeriveIden)]
enum IssuanceReturn {
    Table,
    Id,
    IssuanceId,
    ReturnedAt,
    HasCharger,
    ReturnedAmmo,
    Observations,
}

