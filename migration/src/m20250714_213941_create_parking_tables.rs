use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250618_113709_create_base_tables::Base;

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
                    .table(Position::Table)
                    .if_not_exists()
                    .col(pk_auto(Position::Id))
                    .col(string(Position::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Calibre::Table)
                    .if_not_exists()
                    .col(pk_auto(Calibre::Id))
                    .col(string(Calibre::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AssignanceTime::Table)
                    .if_not_exists()
                    .col(pk_auto(AssignanceTime::Id))
                    .col(string(AssignanceTime::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        let weapon_types = ["Pistola", "Fusil", "Sub-fusil", "Escopeta", "Chaleco"];

        for weapon_type in weapon_types {
            let insert = Query::insert()
                .into_table(WeaponType::Table)
                .columns([WeaponType::Name])
                .values_panic([weapon_type.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

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

        let weapon_brands = [
            "Glock",
            "Beretta",
            "Remington",
            "Browning",
            "Zamorana",
            "Caracal",
            "Colt",
            "H&K",
            "AK",
            "FAL",
        ];

        for weapon_brand in weapon_brands {
            let insert = Query::insert()
                .into_table(WeaponBrand::Table)
                .columns([WeaponBrand::Name])
                .values_panic([weapon_brand.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(WeaponModel::Table)
                    .if_not_exists()
                    .col(pk_auto(WeaponModel::Id))
                    .col(string(WeaponModel::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        let weapon_models = [
            // Glock
            "Glock 17",
            "Glock 19",
            // Beretta
            "FS92",
            "PX4",
            "90TWO",
            // Remington
            "Remington 870",
            "Remington 700",
            // Browning
            "Browning Hi-Power",
            "Browning A5",
            // Zamorana (modelo genérico, si aplica)
            "Zamorana Modelo 1",
            // Caracal
            "Caracal F",
            "Caracal C",
            // Colt
            "Colt 1911",
            "Colt Python",
            // H&K
            "HK USP",
            "HK G36",
            // AK
            "AK-47",
            "AK-103",
            // FAL
            "FN FAL",
            // Modelo original que ya estaba
            "Winchester M1897",
        ];

        for weapon_model in weapon_models {
            let insert = Query::insert()
                .into_table(WeaponModel::Table)
                .columns([WeaponModel::Name])
                .values_panic([weapon_model.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

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
                    .col(date_time(Weapon::EntryAt).default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Weapon::DocumentId).string().null())
                    .col(ColumnDef::new(Weapon::BaseId).integer().not_null())
                    .col(integer(Weapon::CalibreId))
                    .col(integer(Weapon::PositionId))
                    .col(ColumnDef::new(Weapon::ManteinanceAt).date_time().null())
                    .col(boolean(Weapon::HasCharger).default(false))
                    .col(ColumnDef::new(Weapon::Observations).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon-base_id")
                            .from(Weapon::Table, Weapon::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon-calibre_id")
                            .from(Weapon::Table, Weapon::BaseId)
                            .to(Calibre::Table, Calibre::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-weapon-position_id")
                            .from(Weapon::Table, Weapon::PositionId)
                            .to(Position::Table, Position::Id),
                    )
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
                    .col(date_time(Issuance::DateTime).default(Expr::current_timestamp()))
                    .col(string(Issuance::Type))
                    .col(integer(Issuance::AssignanceTimeId))
                    .col(integer(Issuance::AuthById))
                    .col(integer(Issuance::BaseId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance-base_id")
                            .from(Issuance::Table, Issuance::BaseId)
                            .to(Base::Table, Base::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-assignance-time_id")
                            .from(Issuance::Table, Issuance::AssignanceTimeId)
                            .to(AssignanceTime::Table, AssignanceTime::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance-persona_id")
                            .from(Issuance::Table, Issuance::AssignedPersonaId)
                            .to(Persona::Table, Persona::Id),
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
                    .table(IssuanceWeapon::Table)
                    .if_not_exists()
                    .col(pk_auto(IssuanceWeapon::Id))
                    .col(integer(IssuanceWeapon::WeaponId))
                    .col(integer(IssuanceWeapon::IssuanceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance_weapon_weapon_id")
                            .from(IssuanceWeapon::Table, IssuanceWeapon::WeaponId)
                            .to(Weapon::Table, Weapon::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-issuance_weapon_issuance_id")
                            .from(IssuanceWeapon::Table, IssuanceWeapon::IssuanceId)
                            .to(Persona::Table, Persona::Id),
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
                    .col(integer(IssuanceReturn::IssuanceId).unique_key())
                    .col(date_time(IssuanceReturn::ReturnedAt).default(Expr::current_timestamp()))
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
            .drop_table(Table::drop().table(Calibre::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Position::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AssignanceTime::Table).to_owned())
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
}

#[derive(DeriveIden)]
pub enum AssignanceTime {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Calibre {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Position {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Weapon {
    Table,
    Id,
    Photo,
    Serial,
    EntryAt,
    DocumentId,
    CalibreId,
    PositionId,
    ManteinanceAt,
    HasCharger,
    Observations,
    TypeId,
    ModelId,
    BaseId,
}

#[derive(DeriveIden)]
enum Issuance {
    Table,
    Id,
    AssignedPersonaId,
    DateTime,
    Type,
    AssignanceTimeId,
    AuthById,
    BaseId,
}

#[derive(DeriveIden)]
enum IssuanceWeapon {
    Table,
    Id,
    WeaponId,
    IssuanceId,
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
