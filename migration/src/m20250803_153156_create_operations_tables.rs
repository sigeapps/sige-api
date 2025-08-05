use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::{EnumIter, Iterable},
};

use crate::m20250419_000005_create_organism_table::Organism;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub struct InclusionTypeEnum;

#[derive(DeriveIden, EnumIter)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "inclusion_type_enum"
)]
pub enum InclusionType {
    #[sea_orm(iden = "flagrant")]
    Flagrant,
    #[sea_orm(iden = "complaint")]
    Complaint,
    #[sea_orm(iden = "init_order")]
    InitOrder,
    #[sea_orm(iden = "investigation")]
    Investigation,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Creating inclusion records table");

        manager
            .create_type(
                Type::create()
                    .as_enum(InclusionTypeEnum)
                    .values(InclusionType::iter())
                    .to_owned(),
            )
            .await?;

        // 1. Tabla principal de expedientes de inclusión
        manager
            .create_table(
                Table::create()
                    .table(InclusionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InclusionRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::RecordId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::Type)
                            .custom(InclusionTypeEnum)
                            .not_null(),
                    )
                    .col(ColumnDef::new(InclusionRecords::Reason).text().not_null())
                    .col(
                        ColumnDef::new(InclusionRecords::DateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::AuthPersonaId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::EvidencesFilePath)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(InclusionRecords::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating complainants table");

        // 2. Tabla de denunciantes
        manager
            .create_table(
                Table::create()
                    .table(Complainants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Complainants::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Complainants::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Complainants::Ci)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Complainants::Name)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Complainants::LastName)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Complainants::Age).integer())
                    .col(ColumnDef::new(Complainants::Phone).string().string_len(20))
                    .col(ColumnDef::new(Complainants::BirthDate).date())
                    .col(
                        ColumnDef::new(Complainants::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_complainants_inclusion_record")
                            .from(Complainants::Table, Complainants::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating objects involved table");
        // 3. Tabla de objetos involucrados
        manager
            .create_table(
                Table::create()
                    .table(InvolvedObjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InvolvedObjects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InvolvedObjects::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvolvedObjects::Type)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvolvedObjects::Serial)
                            .string()
                            .string_len(100),
                    )
                    .col(ColumnDef::new(InvolvedObjects::PhysicalSigns).text())
                    .col(
                        ColumnDef::new(InvolvedObjects::Description)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InvolvedObjects::PhotoPath)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(InvolvedObjects::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_objects_inclusion_record")
                            .from(InvolvedObjects::Table, InvolvedObjects::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating diligencies table");

        // 4. Tabla de diligencias
        manager
            .create_table(
                Table::create()
                    .table(Diligencies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Diligencies::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Diligencies::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Diligencies::DateTime).date_time().not_null())
                    .col(
                        ColumnDef::new(Diligencies::BrigadeBossId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Diligencies::Nomenclature)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Diligencies::Inspections).text())
                    .col(ColumnDef::new(Diligencies::Evidences).text())
                    .col(
                        ColumnDef::new(Diligencies::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_diligencies_inclusion_record")
                            .from(Diligencies::Table, Diligencies::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating expressive expertises table");

        // 5. Tabla de experticias técnicas
        manager
            .create_table(
                Table::create()
                    .table(TechnicalExpertises::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TechnicalExpertises::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TechnicalExpertises::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TechnicalExpertises::DateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TechnicalExpertises::EvidenceType)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TechnicalExpertises::Description)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TechnicalExpertises::ChainOfCustody).text())
                    .col(ColumnDef::new(TechnicalExpertises::ResultsDateTime).date_time())
                    .col(ColumnDef::new(TechnicalExpertises::Results).text())
                    .col(ColumnDef::new(TechnicalExpertises::DocumentsPath).string_len(255))
                    .col(
                        ColumnDef::new(TechnicalExpertises::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expertises_inclusion_record")
                            .from(
                                TechnicalExpertises::Table,
                                TechnicalExpertises::InclusionRecordId,
                            )
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating arrests table");
        // 6. Tabla de detenciones
        manager
            .create_table(
                Table::create()
                    .table(Arrests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Arrests::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Arrests::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Arrests::DateTime).date_time().not_null())
                    .col(
                        ColumnDef::new(Arrests::DetainedPersonId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Arrests::ArrestType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Arrests::Reason).text().not_null())
                    .col(ColumnDef::new(Arrests::PhotoPath).string_len(255))
                    .col(
                        ColumnDef::new(Arrests::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_arrests_inclusion_record")
                            .from(Arrests::Table, Arrests::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating incauted materials table");
        // 7. Tabla de material incautado
        manager
            .create_table(
                Table::create()
                    .table(ConfiscatedItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConfiscatedItems::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConfiscatedItems::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfiscatedItems::DateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfiscatedItems::ItemType)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConfiscatedItems::Description)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ConfiscatedItems::PhotoPath).string_len(255))
                    .col(
                        ColumnDef::new(ConfiscatedItems::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_confiscated_inclusion_record")
                            .from(ConfiscatedItems::Table, ConfiscatedItems::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 9. Tabla de presentaciones judiciales
        manager
            .create_table(
                Table::create()
                    .table(JudicialPresentations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JudicialPresentations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::DateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::AuthorityAssigned)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::AuthorityName)
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::AuthorityPhone)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(JudicialPresentations::AssignedCourt).string_len(100))
                    .col(ColumnDef::new(JudicialPresentations::AuthorityDecision).text())
                    .col(
                        ColumnDef::new(JudicialPresentations::ConfinementPlace)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JudicialPresentations::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_presentations_inclusion_record")
                            .from(
                                JudicialPresentations::Table,
                                JudicialPresentations::InclusionRecordId,
                            )
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating flagrants table");
        // 10. Tabla de flagrantes
        manager
            .create_table(
                Table::create()
                    .table(Flagrants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Flagrants::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Flagrants::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Flagrants::Latitude).decimal())
                    .col(ColumnDef::new(Flagrants::Longitude).decimal())
                    .col(
                        ColumnDef::new(Flagrants::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_flagrants_inclusion_record")
                            .from(Flagrants::Table, Flagrants::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating inclusion records personas table");
        // 11. Tabla de relación entre expedientes de inclusión y personas
        manager
            .create_table(
                Table::create()
                    .table(InclusionRecordsPersonas::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InclusionRecordsPersonas::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsPersonas::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsPersonas::PersonaId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsPersonas::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_records_personas_inclusion_record")
                            .from(
                                InclusionRecordsPersonas::Table,
                                InclusionRecordsPersonas::InclusionRecordId,
                            )
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating investigations order table");
        // 12. Tabla de órdenes de investigación
        manager
            .create_table(
                Table::create()
                    .table(InitOrders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InitOrders::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InitOrders::AnnotatedId).integer().not_null())
                    .col(ColumnDef::new(InitOrders::OrganismId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_init_orders_organism")
                            .from(InitOrders::Table, InitOrders::OrganismId)
                            .to(Organism::Table, Organism::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(InitOrders::Type).string_len(100).not_null())
                    .col(ColumnDef::new(InitOrders::Instruction).text().not_null())
                    .col(
                        ColumnDef::new(InitOrders::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_init_orders_inclusion_record")
                            .from(InitOrders::Table, InitOrders::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating investigations table");
        // 13. Tabla de investigaciones
        manager
            .create_table(
                Table::create()
                    .table(Investigations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Investigations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Investigations::AnnotatedInvestigationId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Investigations::Source)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Investigations::Classification)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Investigations::Type)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Investigations::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_investigations_inclusion_record")
                            .from(Investigations::Table, Investigations::InclusionRecordId)
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        println!("Creating inclusion records accuseds table");
        // 14. Tabla de relación entre expedientes de inclusión y acusados
        manager
            .create_table(
                Table::create()
                    .table(InclusionRecordsAccuseds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InclusionRecordsAccuseds::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsAccuseds::InclusionRecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsAccuseds::AccusedId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(InclusionRecordsAccuseds::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_records_accuseds_inclusion_record")
                            .from(
                                InclusionRecordsAccuseds::Table,
                                InclusionRecordsAccuseds::InclusionRecordId,
                            )
                            .to(InclusionRecords::Table, InclusionRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inclusion_type_date")
                    .table(InclusionRecords::Table)
                    .col(InclusionRecords::Type)
                    .col(InclusionRecords::DateTime)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_complainant_name")
                    .table(Complainants::Table)
                    .col(Complainants::Name)
                    .col(Complainants::LastName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(
                Type::drop()
                    .name(InclusionTypeEnum)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(InclusionRecordsAccuseds::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Investigations::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(InitOrders::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(InclusionRecordsPersonas::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Flagrants::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(JudicialPresentations::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ConfiscatedItems::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Arrests::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TechnicalExpertises::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Diligencies::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(InvolvedObjects::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Complainants::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(InclusionRecords::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

// Definiciones de las tablas usando enums
#[derive(DeriveIden)]
enum InclusionRecords {
    Table,
    Id,
    RecordId,
    Type,
    Reason,
    DateTime,
    AuthPersonaId,
    CreatedAt,
    UpdatedAt,
    EvidencesFilePath,
}

#[derive(DeriveIden)]
enum Flagrants {
    Table,
    Id,
    InclusionRecordId,
    Latitude,
    Longitude,
    CreatedAt,
}

#[derive(DeriveIden)]
enum InclusionRecordsPersonas {
    Table,
    Id,
    InclusionRecordId,
    PersonaId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum InitOrders {
    Table,
    Id,
    InclusionRecordId,
    AnnotatedId,
    OrganismId,
    Type,
    Instruction,
}

#[derive(DeriveIden)]
enum Investigations {
    Table,
    Id,
    InclusionRecordId,
    AnnotatedInvestigationId,
    Source,
    Classification,
    Type,
}

#[derive(DeriveIden)]
enum Complainants {
    Table,
    Id,
    InclusionRecordId,
    Ci,
    Name,
    LastName,
    Age,
    BirthDate,
    Phone,
    CreatedAt,
}

#[derive(DeriveIden)]
enum InvolvedObjects {
    Table,
    Id,
    InclusionRecordId,
    Type,
    Serial,
    PhysicalSigns,
    Description,
    PhotoPath,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Diligencies {
    Table,
    Id,
    InclusionRecordId,
    DateTime,
    BrigadeBossId,
    Nomenclature,
    Inspections,
    Evidences,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TechnicalExpertises {
    Table,
    Id,
    InclusionRecordId,
    DateTime,
    EvidenceType,
    Description,
    ChainOfCustody,
    ResultsDateTime,
    Results,
    DocumentsPath,
    CreatedAt,
}

#[derive(DeriveIden)]
enum InclusionRecordsAccuseds {
    Table,
    Id,
    InclusionRecordId,
    AccusedId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Arrests {
    Table,
    Id,
    InclusionRecordId,
    DateTime,
    DetainedPersonId,
    ArrestType,
    Reason,
    PhotoPath,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ConfiscatedItems {
    Table,
    Id,
    InclusionRecordId,
    DateTime,
    ItemType,
    Description,
    PhotoPath,
    CreatedAt,
}

#[derive(DeriveIden)]
enum JudicialPresentations {
    Table,
    Id,
    InclusionRecordId,
    DateTime,
    AuthorityAssigned,
    AuthorityName,
    AuthorityPhone,
    AssignedCourt,
    AuthorityDecision,
    ConfinementPlace,
    CreatedAt,
}
