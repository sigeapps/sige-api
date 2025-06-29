use domain::entities::{base, division, persona_state, plate_persona, state};
use sea_orm::*;
use std::sync::Arc;

use crate::dtos::personal::plate::{
    persona::{ModifyPersonaResponse, PlatePersona},
    GetPlateDTO, PlateRequestDTO, PlateResponseDTO,
};

#[derive(Debug, Clone)]
pub struct PlateService {
    pub db: Arc<DatabaseConnection>,
}

impl PlateService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        PlateService { db }
    }

    pub async fn create(self, dto: PlateRequestDTO) -> Result<i32, DbErr> {
        let transaction = self.db.begin().await?;

        let id = dto.plate.into_active_model().insert(&transaction).await?.id;

        for persona in dto.modified_personas {
            let mut active_model = persona.into_active_model();

            active_model.plate_id = Set(id);

            active_model.clone().insert(&transaction).await?;

            active_model.into_persona()?.update(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn find_by_id(self, id: i32) -> Result<Option<PlateResponseDTO>, DbErr> {
        let plate = domain::entities::plate::Entity::find_by_id(id)
            .left_join(base::Entity)
            .left_join(state::Entity)
            .left_join(division::Entity)
            .into_partial_model::<GetPlateDTO>()
            .one(&*self.db)
            .await?;

        // Esta sección fue reescrita para evitar errores con into_partial_model.
        // Se obtienen los plate_persona, luego se buscan los estados y la persona asociada, y se mapea a ModifyPersonaResponse.
        // TODO: Refactorizar para que no se tenga que hacer un use de los DTOs en el mismo archivo

        use crate::dtos::personal::persona::SimplePersonaResponseDTO;
        use crate::dtos::prevention::lookup::GetPersonaStateDTO;

        let plate_personas = domain::entities::plate_persona::Entity::find()
            .filter(domain::entities::plate_persona::Column::PlateId.eq(id))
            .all(&*self.db)
            .await?;

        let mut personas = Vec::new();

        for plate_persona in plate_personas {
            // Buscar la persona asociada
            let persona = domain::entities::persona::Entity::find_by_id(plate_persona.persona_id)
                .into_partial_model::<SimplePersonaResponseDTO>()
                .one(&*self.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            // Buscar el estado nuevo
            let new_state = persona_state::Entity::find_by_id(plate_persona.new_state_id)
                .into_partial_model::<GetPersonaStateDTO>()
                .one(&*self.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            // Buscar el estado viejo
            let old_state = persona_state::Entity::find_by_id(plate_persona.old_state_id)
                .into_partial_model::<GetPersonaStateDTO>()
                .one(&*self.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            personas.push(ModifyPersonaResponse {
                new_state: new_state,
                old_state: old_state,
                persona: persona,
            });
        }

        match plate {
            Some(plate) => Ok(Some(PlateResponseDTO {
                plate,
                modified_personas: personas,
            })),
            None => Ok(None),
        }
    }
}
