use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20250419_000002_create_user_table::User,
    m20250611_094810_create_persona_table::PersonaSituation,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

// TODO: Añadir constantes para esta migración

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use crate::m20250611_094810_create_persona_table::Persona;

        let db = manager.get_connection();

        println!("Seeding persona example with CI: V00000000");

        // Verificar si la persona ya existe
        let persona_exists = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM persona WHERE ci = 'V00000000'".to_string(),
            ))
            .await?;

        if persona_exists.is_none() || persona_exists.unwrap().try_get::<i64>("", "count")? == 0 {
            // Insertar persona de ejemplo
            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(Persona::Table)
                        .columns([
                            Persona::Ci,
                            Persona::Name,
                            Persona::LastName,
                            Persona::Birthdate,
                            Persona::Age,
                            Persona::Birthplace,
                            Persona::Address,
                            Persona::Phone,
                            Persona::Email,
                            Persona::Genre,
                            Persona::StatusCivil,
                            Persona::BankAccount,
                            Persona::HomelandCi,
                            Persona::VehicleLicense,
                        ])
                        .values_panic([
                            "V00000000".into(),
                            "Solicitud".into(),
                            "Propia".into(),
                            "1985-03-15".into(),
                            38.into(),
                            "Caracas, Distrito Capital".into(),
                            "Helicoide".into(),
                            "0414-1234567".into(),
                            "juan.perez@gmail.com".into(),
                            "Masculino".into(),
                            "Soltero".into(),
                            "0102-3456-78901234567890".into(),
                            "000000000".into(),
                            "0000000000000000000".into(),
                        ])
                        .to_owned(),
                )
                .await?;

            println!("Persona with CI V00000000 created successfully");

            // Obtener el ID de la persona recién creada para insertar datos relacionados
            let persona_row = db
                .query_one(Statement::from_string(
                    manager.get_database_backend(),
                    "SELECT id FROM persona WHERE ci = 'V00000000'".to_string(),
                ))
                .await?;

            let persona_id = if let Some(row) = persona_row {
                row.try_get::<i32>("", "id")?
            } else {
                return Err(DbErr::Custom(
                    "No se pudo encontrar la persona creada".to_string(),
                ));
            };

            // Insertar características físicas
            use crate::m20250611_094810_create_persona_table::PersonaTraits;

            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(PersonaTraits::Table)
                        .columns([
                            PersonaTraits::PersonaId,
                            PersonaTraits::Build,
                            PersonaTraits::Height,
                            PersonaTraits::SkinColor,
                            PersonaTraits::HasTattoos,
                            PersonaTraits::EyesColor,
                            PersonaTraits::EyesType,
                            PersonaTraits::HairColor,
                            PersonaTraits::HairType,
                            PersonaTraits::EyebrowType,
                            PersonaTraits::NoseType,
                            PersonaTraits::FaceType,
                            PersonaTraits::LipsType,
                            PersonaTraits::HandsType,
                        ])
                        .values_panic([
                            persona_id.into(),
                            "Atlético".into(),
                            175.into(),
                            "Moreno claro".into(),
                            false.into(),
                            "Marrón".into(),
                            "Almendrados".into(),
                            "Negro".into(),
                            "Liso".into(),
                            "Pobladas".into(),
                            "Recta".into(),
                            "Ovalada".into(),
                            "Medianos".into(),
                            "Normales".into(),
                        ])
                        .to_owned(),
                )
                .await?;

            println!("PersonaTraits created for persona with CI V00000000");

            // Insertar información de salud
            use crate::m20250611_094810_create_persona_table::PersonaHealth;

            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(PersonaHealth::Table)
                        .columns([
                            PersonaHealth::PersonaId,
                            PersonaHealth::BloodGroup,
                            PersonaHealth::HasFractures,
                        ])
                        .values_panic([persona_id.into(), "O+".into(), false.into()])
                        .to_owned(),
                )
                .await?;

            println!("PersonaHealth created for persona with CI V00000000");

            // Insertar un registro básico
            use crate::m20250611_094810_create_persona_table::PersonaRecord;

            manager
                .exec_stmt(
                    Query::insert()
                        .into_table(PersonaRecord::Table)
                        .columns([
                            PersonaRecord::PersonaId,
                            PersonaRecord::Name,
                            PersonaRecord::Type,
                        ])
                        .values_panic([
                            persona_id.into(),
                            "Antecedentes Penales".into(),
                            "Judicial".into(),
                        ])
                        .to_owned(),
                )
                .await?;

            println!("PersonaRecord created for persona with CI V00000000");
        } else {
            println!("Persona with CI V00000000 already exists, skipping...");
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use crate::m20250611_094810_create_persona_table::{
            Persona, PersonaHealth, PersonaRecord, PersonaTraits,
        };
        use sea_orm_migration::sea_orm::Statement;

        let db = manager.get_connection();

        // Obtener el ID de la persona para eliminar registros relacionados
        let persona_row = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT id FROM persona WHERE ci = 'V00000000'".to_string(),
            ))
            .await?;

        if let Some(row) = persona_row {
            let persona_id = row.try_get::<i32>("", "id")?;

            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(PersonaTraits::Table)
                        .and_where(Expr::col(PersonaTraits::PersonaId).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            // Eliminar registros relacionados primero
            // Eliminar PersonaRecord
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(PersonaRecord::Table)
                        .and_where(Expr::col(PersonaRecord::PersonaId).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            // Eliminar PersonaHealth
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(PersonaHealth::Table)
                        .and_where(Expr::col(PersonaHealth::PersonaId).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            // Eliminar usuarios que referencian esta persona
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(User::Table)
                        .and_where(Expr::col(User::PersonaId).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            // Eliminar PersonaSituation
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(PersonaSituation::Table)
                        .and_where(Expr::col(PersonaSituation::PersonaId).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            // Necesitamos eliminar las situaciones asociadas a la persona

            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(PersonaSituation::Table)
                        .and_where(Expr::col(PersonaSituation::RequestedById).eq(persona_id))
                        .to_owned(),
                )
                .await?;

            println!("Deleting main persona table");

            // Finalmente eliminar la persona
            manager
                .exec_stmt(
                    Query::delete()
                        .from_table(Persona::Table)
                        .and_where(Expr::col(Persona::Ci).eq("V00000000"))
                        .to_owned(),
                )
                .await?;

            println!("Persona with CI V00000000 and all related data removed");
        } else {
            println!("Persona with CI V00000000 not found, nothing to remove");
        }

        Ok(())
    }
}
