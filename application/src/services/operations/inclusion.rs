use sea_orm::*;

use crate::{api::ApiContext, dtos::operations::inclusion::CreateInclusion};

pub struct InclusionService;

impl InclusionService {
    pub async fn create_inclusion(
        ctx: ApiContext,
        inclusion: CreateInclusion,
    ) -> Result<i32, DbErr> {
        // Extraemos la base y luego insertamos por tipo
        let base = match inclusion.clone() {
            CreateInclusion::Flagrant { base, .. } => base,
            CreateInclusion::Complainant { base, .. } => base,
            CreateInclusion::InitOrder { base, .. } => base,
            CreateInclusion::Investigation { base, .. } => base,
        };

        let conn = ctx.db.begin().await?;

        // Guardar el registro de inclusión principal
        let inclusion_record = base.record.into_active_model().insert(&conn).await?;

        // Guardar objetos involucrados
        for obj in base.involved_objects {
            let mut involved_object = obj.into_active_model();

            involved_object.inclusion_record_id = Set(inclusion_record.id);
            involved_object.insert(&conn).await?;
        }

        // Guardar diligencias
        for d in base.diligencies {
            let mut diligence = d.into_active_model();
            diligence.inclusion_record_id = Set(inclusion_record.id);
            diligence.insert(&conn).await?;
        }

        // Guardar experticias técnicas
        for te in base.technical_expertises {
            let mut technical_expertise = te.into_active_model();
            technical_expertise.inclusion_record_id = Set(inclusion_record.id);
            technical_expertise.insert(&conn).await?;
        }

        // Guardar arrestos
        for arrest in base.arrests {
            let mut arrest_model = arrest.into_active_model();
            arrest_model.inclusion_record_id = Set(inclusion_record.id);
            arrest_model.insert(&conn).await?;
        }

        // Guardar items confiscados
        for item in base.confiscated_items {
            let mut confiscated_item = item.into_active_model();
            confiscated_item.inclusion_record_id = Set(inclusion_record.id);
            confiscated_item.insert(&conn).await?;
        }

        // Guardar presentación judicial (asumiendo que es uno solo)
        let mut judicial_presentation = base.judicial_presentations.into_active_model();
        judicial_presentation.inclusion_record_id = Set(inclusion_record.id);
        judicial_presentation.insert(&conn).await?;

        match inclusion {
            CreateInclusion::Flagrant { flagrant, .. } => {
                let mut flagrant_model = flagrant.into_active_model();
                flagrant_model.inclusion_record_id = Set(inclusion_record.id);
                flagrant_model.insert(&conn).await?;
            }
            CreateInclusion::Complainant { complainant, .. } => {
                let mut complainant_model = complainant.into_active_model();
                complainant_model.inclusion_record_id = Set(inclusion_record.id);
                complainant_model.insert(&conn).await?;
            }
            CreateInclusion::InitOrder { init_order, .. } => {
                let mut init_order_model = init_order.into_active_model();
                init_order_model.inclusion_record_id = Set(inclusion_record.id);
                init_order_model.insert(&conn).await?;
            }
            CreateInclusion::Investigation { investigation, .. } => {
                let mut investigation_model = investigation.into_active_model();
                investigation_model.inclusion_record_id = Set(inclusion_record.id);
                investigation_model.insert(&conn).await?;
            }
        }

        conn.commit().await?;

        Ok(inclusion_record.id)
    }
}
