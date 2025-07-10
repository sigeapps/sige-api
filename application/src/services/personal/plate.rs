use domain::entities::{base, division, persona_state, plate, state};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{FilterByClaims, HasBaseId, UserClaims, UserStamp},
    dtos::{
        personal::plate::{
            persona::{ModifyPersonaResponse, PlatePersona},
            GetPlateDTO, PlateRequestDTO, PlateResponseDTO,
        },
        CommonQueryFilterDTO,
    },
    impl_filter_by_claims,
};

impl HasBaseId for plate::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(plate, BaseId);

#[derive(Debug, Clone)]
pub struct PlateService {}
impl PlateService {
    pub async fn create(ctx: ApiContext, dto: PlateRequestDTO) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        let id = dto
            .plate
            .into_active_model()
            .stamp_user(ctx.claims)
            .insert(&transaction)
            .await?
            .id;

        for persona in dto.modified_personas {
            let mut active_model = persona.into_active_model();

            active_model.plate_id = Set(id);

            active_model.clone().insert(&transaction).await?;

            active_model.into_persona()?.update(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<Option<PlateResponseDTO>, DbErr> {
        let plate = domain::entities::plate::Entity::find_by_id(id)
            .filter_by_claims(ctx.claims)
            .left_join(base::Entity)
            .left_join(state::Entity)
            .left_join(division::Entity)
            .into_partial_model::<GetPlateDTO>()
            .one(&ctx.db)
            .await?;

        // Esta sección fue reescrita para evitar errores con into_partial_model.
        // Se obtienen los plate_persona, luego se buscan los estados y la persona asociada, y se mapea a ModifyPersonaResponse.
        // TODO: Refactorizar para que no se tenga que hacer un use de los DTOs en el mismo archivo

        use crate::dtos::personal::persona::SimplePersonaResponseDTO;
        use crate::dtos::prevention::lookup::GetPersonaStateDTO;

        let plate_personas = domain::entities::plate_persona::Entity::find()
            .filter(domain::entities::plate_persona::Column::PlateId.eq(id))
            .all(&ctx.db)
            .await?;

        let mut personas = Vec::new();

        for plate_persona in plate_personas {
            // Buscar la persona asociada
            let persona = domain::entities::persona::Entity::find_by_id(plate_persona.persona_id)
                .into_partial_model::<SimplePersonaResponseDTO>()
                .one(&ctx.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            // Buscar el estado nuevo
            let new_state = persona_state::Entity::find_by_id(plate_persona.new_state_id)
                .into_partial_model::<GetPersonaStateDTO>()
                .one(&ctx.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            // Buscar el estado viejo
            let old_state = persona_state::Entity::find_by_id(plate_persona.old_state_id)
                .into_partial_model::<GetPersonaStateDTO>()
                .one(&ctx.db)
                .await?
                .ok_or(DbErr::RecordNotFound("record not found".to_string()))?;

            personas.push(ModifyPersonaResponse {
                new_state,
                old_state,
                persona,
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

    pub async fn find(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetPlateDTO>, DbErr> {
        let mut query = domain::entities::plate::Entity::find()
            .filter_by_claims(ctx.claims)
            .left_join(base::Entity)
            .left_join(state::Entity)
            .left_join(division::Entity);

        if let Some(search) = filter.search {
            query = query.filter(
                domain::entities::plate::Column::Id
                    .contains(&search)
                    .or(domain::entities::base::Column::Name.contains(&search))
                    .or(domain::entities::state::Column::Name.contains(&search))
                    .or(domain::entities::division::Column::Name.contains(&search)),
            );
        }

        let plates = query
            .into_partial_model::<GetPlateDTO>()
            .all(&ctx.db)
            .await?;

        Ok(plates)
    }
}
