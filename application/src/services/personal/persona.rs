use sea_orm::*;
use std::sync::Arc;

use crate::dtos::personal::persona::CreatePersonaDTO;

#[derive(Debug, Clone)]
pub struct PersonaService {
    pub db: Arc<DatabaseConnection>,
}

impl PersonaService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        PersonaService { db }
    }

    pub async fn create(self, mut dto: CreatePersonaDTO) -> Result<i32, DbErr> {
        let transaction = self.db.begin().await?;

        dto.personal.others = dto.others.others;

        let persona_id = dto
            .personal
            .into_active_model()
            .insert(&transaction)
            .await?
            .id;

        dto.traits.persona_id = persona_id;

        dto.traits.into_active_model().insert(&transaction).await?;

        dto.conyuge.persona_id = persona_id;

        dto.conyuge.into_active_model().insert(&transaction).await?;

        dto.health.persona_id = persona_id;

        dto.health.into_active_model().insert(&transaction).await?;

        async {
            for mut operational in dto.operational {
                operational.persona_id = persona_id;

                operational.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut relative in dto.relatives {
                relative.persona_id = persona_id;

                relative.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut course in dto.courses {
                course.persona_id = persona_id;

                course.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut record in dto.records {
                record.persona_id = persona_id;

                record.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut education in dto.education {
                education.persona_id = persona_id;

                education.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut labor in dto.work_experiencies {
                labor.persona_id = persona_id;

                labor.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        async {
            for mut children in dto.childrens {
                children.persona_id = persona_id;

                children.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(persona_id)
    }
}
