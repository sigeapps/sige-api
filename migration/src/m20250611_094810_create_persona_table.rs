use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Persona::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Persona::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Persona::Ci).string().not_null())
                    .col(ColumnDef::new(Persona::FrontPhoto).string().null())
                    .col(ColumnDef::new(Persona::BackPhoto).string().null())
                    .col(ColumnDef::new(Persona::PassportNumber).string().not_null())
                    .col(
                        ColumnDef::new(Persona::PassportExpiration)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Persona::PassportYearsValid)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Persona::Name).string().not_null())
                    .col(ColumnDef::new(Persona::LastName).string().not_null())
                    .col(ColumnDef::new(Persona::Birthdate).string().not_null())
                    .col(ColumnDef::new(Persona::Age).integer().not_null())
                    .col(ColumnDef::new(Persona::Birthplace).string().not_null())
                    .col(ColumnDef::new(Persona::Address).string().not_null())
                    .col(ColumnDef::new(Persona::Phone).string().not_null())
                    .col(ColumnDef::new(Persona::Coordinates).string().null())
                    .col(ColumnDef::new(Persona::Genre).string().not_null())
                    .col(ColumnDef::new(Persona::StatusCivil).string().not_null())
                    .col(ColumnDef::new(Persona::BankAccount).string().not_null())
                    .col(ColumnDef::new(Persona::HomelandCi).string().not_null())
                    .col(ColumnDef::new(Persona::VehicleLicense).string().not_null())
                    .col(ColumnDef::new(Persona::Others).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaTraits::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaTraits::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaTraits::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_traits_persona")
                            .from(PersonaTraits::Table, PersonaTraits::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaTraits::Build).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::Height).integer().not_null())
                    .col(ColumnDef::new(PersonaTraits::SkinColor).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::HasTattoos).boolean().null())
                    .col(ColumnDef::new(PersonaTraits::TattoosPhoto).string().null())
                    .col(ColumnDef::new(PersonaTraits::EyesColor).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::EyesType).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::HairColor).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::HairType).string().not_null())
                    .col(
                        ColumnDef::new(PersonaTraits::EyebrowType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaTraits::NoseType).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::FaceType).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::LipsType).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::HandsType).string().not_null())
                    .col(ColumnDef::new(PersonaTraits::Others).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaConyuge::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaConyuge::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaConyuge::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_conyuge_persona")
                            .from(PersonaConyuge::Table, PersonaConyuge::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaConyuge::Name).string().not_null())
                    .col(ColumnDef::new(PersonaConyuge::LastName).string().not_null())
                    .col(ColumnDef::new(PersonaConyuge::Ci).string().not_null())
                    .col(ColumnDef::new(PersonaConyuge::Birthdate).date().not_null())
                    .col(ColumnDef::new(PersonaConyuge::Age).integer().not_null())
                    .col(ColumnDef::new(PersonaConyuge::Phone).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaChildren::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaChildren::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaChildren::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_children_persona")
                            .from(PersonaChildren::Table, PersonaChildren::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaChildren::Name).string().not_null())
                    .col(
                        ColumnDef::new(PersonaChildren::LastName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaChildren::Birthdate).date().not_null())
                    .col(ColumnDef::new(PersonaChildren::Age).integer().not_null())
                    .col(ColumnDef::new(PersonaChildren::Grade).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaRelative::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaRelative::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaRelative::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_relative_persona")
                            .from(PersonaRelative::Table, PersonaRelative::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaRelative::RelationshipId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaRelative::Name).string().not_null())
                    .col(
                        ColumnDef::new(PersonaRelative::LastName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaRelative::Birthdate).date().not_null())
                    .col(ColumnDef::new(PersonaRelative::Age).integer().not_null())
                    .col(ColumnDef::new(PersonaRelative::Phone).string().not_null())
                    .col(ColumnDef::new(PersonaRelative::Address).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaEducation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaEducation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaEducation::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_education_persona")
                            .from(PersonaEducation::Table, PersonaEducation::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaEducation::Grade).string().not_null())
                    .col(
                        ColumnDef::new(PersonaEducation::InstitutionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_education_institution")
                            .from(PersonaEducation::Table, PersonaEducation::InstitutionId)
                            .to(Institution::Table, Institution::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaEducation::ProfessionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_education_profession")
                            .from(PersonaEducation::Table, PersonaEducation::ProfessionId)
                            .to(Profession::Table, Profession::Id),
                    )
                    .col(ColumnDef::new(PersonaEducation::EndDate).date().not_null())
                    .col(ColumnDef::new(PersonaEducation::Photo).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaCourse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaCourse::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaCourse::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_course_persona")
                            .from(PersonaCourse::Table, PersonaCourse::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaCourse::Name).string().not_null())
                    .col(
                        ColumnDef::new(PersonaCourse::InstitutionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_course_institution")
                            .from(PersonaCourse::Table, PersonaCourse::InstitutionId)
                            .to(Institution::Table, Institution::Id),
                    )
                    .col(ColumnDef::new(PersonaCourse::Date).date().not_null())
                    .col(ColumnDef::new(PersonaCourse::Document).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaWorkExperience::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaWorkExperience::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::OrganismId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_work_experience_organism")
                            .from(
                                PersonaWorkExperience::Table,
                                PersonaWorkExperience::OrganismId,
                            )
                            .to(Organism::Table, Organism::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::ChargeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_work_experience_charge")
                            .from(
                                PersonaWorkExperience::Table,
                                PersonaWorkExperience::ChargeId,
                            )
                            .to(Charge::Table, Charge::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_work_experiencie_persona")
                            .from(
                                PersonaWorkExperience::Table,
                                PersonaWorkExperience::PersonaId,
                            )
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::HierarchyId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_work_experience_hierarchy")
                            .from(
                                PersonaWorkExperience::Table,
                                PersonaWorkExperience::HierarchyId,
                            )
                            .to(Hierarchy::Table, Hierarchy::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::BossPhone)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::IsActive)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PersonaWorkExperience::Description)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaWorkExperience::Photo).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaHealth::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaHealth::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaHealth::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_health_persona")
                            .from(PersonaHealth::Table, PersonaHealth::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaHealth::BloodGroup)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaHealth::Allergies).string().null())
                    .col(ColumnDef::new(PersonaHealth::Operations).string().null())
                    .col(
                        ColumnDef::new(PersonaHealth::HasFractures)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaHealth::Fractures).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaOperational::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaOperational::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaOperational::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_operational_persona")
                            .from(PersonaOperational::Table, PersonaOperational::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaOperational::OrganismId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_operational_organism")
                            .from(PersonaOperational::Table, PersonaOperational::OrganismId)
                            .to(Organism::Table, Organism::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaOperational::ChargeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_operational_charge")
                            .from(PersonaOperational::Table, PersonaOperational::ChargeId)
                            .to(Charge::Table, Charge::Id),
                    )
                    .col(
                        ColumnDef::new(PersonaOperational::StartAt)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaOperational::EndAt).date().not_null())
                    .col(
                        ColumnDef::new(PersonaOperational::Time)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PersonaOperational::Phone)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PersonaOperational::File).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonaRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonaRecord::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PersonaRecord::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persona_record_persona")
                            .from(PersonaRecord::Table, PersonaRecord::PersonaId)
                            .to(Persona::Table, Persona::Id),
                    )
                    .col(ColumnDef::new(PersonaRecord::Name).string().not_null())
                    .col(ColumnDef::new(PersonaRecord::Type).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PersonaRecord::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaTraits::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaWorkExperience::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaChildren::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaRelative::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaOperational::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaConyuge::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(PersonaHealth::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .cascade()
                    .table(Persona::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Persona {
    Table,
    Id,
    Ci,
    FrontPhoto,
    BackPhoto,
    PassportNumber,
    PassportExpiration,
    PassportYearsValid,
    Name,
    LastName,
    Birthdate,
    Age,
    Birthplace,
    Address,
    Phone,
    Coordinates,
    Genre,
    StatusCivil,
    BankAccount,
    HomelandCi,
    VehicleLicense,
    Others,
}

#[derive(DeriveIden)]
enum PersonaTraits {
    Table,
    Id,
    PersonaId,
    Build,
    Height,
    SkinColor,
    HasTattoos,
    TattoosPhoto,
    EyesColor,
    EyesType,
    HairColor,
    HairType,
    EyebrowType,
    NoseType,
    FaceType,
    LipsType,
    HandsType,
    Others,
}

#[derive(DeriveIden)]
enum PersonaConyuge {
    Table,
    Id,
    PersonaId,
    Ci,
    Name,
    LastName,
    Birthdate,
    Age,
    Phone,
    Address,
}

#[derive(DeriveIden)]
enum PersonaChildren {
    Table,
    Id,
    PersonaId,
    Name,
    LastName,
    Birthdate,
    Age,
    Grade,
}

#[derive(DeriveIden)]
enum PersonaRelative {
    Table,
    Id,
    PersonaId,
    RelationshipId,
    Name,
    LastName,
    Birthdate,
    Age,
    Phone,
    Address,
}

#[derive(DeriveIden)]
enum PersonaEducation {
    Table,
    Id,
    PersonaId,
    Grade,
    InstitutionId,
    ProfessionId,
    EndDate,
    Photo,
}

#[derive(DeriveIden)]
enum PersonaCourse {
    Table,
    Id,
    PersonaId,
    Name,
    InstitutionId,
    Date,
    Document,
}

#[derive(DeriveIden)]
enum PersonaWorkExperience {
    Table,
    Id,
    PersonaId,
    OrganismId,
    ChargeId,
    HierarchyId,
    BossPhone,
    IsActive,
    Description,
    Photo,
}

#[derive(DeriveIden)]
enum PersonaHealth {
    Table,
    Id,
    PersonaId,
    BloodGroup,
    Allergies,
    Operations,
    HasFractures,
    Fractures,
}

#[derive(DeriveIden)]
enum PersonaOperational {
    Table,
    Id,
    PersonaId,
    OrganismId,
    ChargeId,
    StartAt,
    EndAt,
    Time,
    Phone,
    File,
}

#[derive(DeriveIden)]
enum PersonaRecord {
    Table,
    Id,
    PersonaId,
    Name,
    Type,
}

#[derive(DeriveIden)]
enum Institution {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Organism {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Charge {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Profession {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Hierarchy {
    Table,
    Id,
}
