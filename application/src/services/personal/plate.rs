use sea_orm::*;
use std::sync::Arc;

use crate::dtos::personal::plate::{persona::PlatePersona, PlateRequestDTO};

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
}
