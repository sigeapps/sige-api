use std::sync::Arc;

use domain::entities::seclusion;
use sea_orm::*;

use crate::dtos::{prevention::seclusion::GetSeclusionDTO, CommonQueryFilterDTO};

pub struct SeclusionService {
    db: Arc<DatabaseConnection>
}

impl SeclusionService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find(self, filter: CommonQueryFilterDTO) -> Result<Vec<GetSeclusionDTO>, DbErr> {
        let mut query = seclusion::Entity::find().limit(filter.limit).offset(filter.offset);

        if let Some(search) = filter.search {
            query = query.filter(Condition::all()
            .add(seclusion::Column::Ci.contains(&search))
            .add(seclusion::Column::ExitReason.contains(&search))
            .add(seclusion::Column::LastName.contains(&search))
            .add(seclusion::Column::FirstName.contains(&search))
            .add(seclusion::Column::Observations.contains(&search))
        )
        }

        query.into_partial_model::<GetSeclusionDTO>().all(&*self.db).await
    }
}
