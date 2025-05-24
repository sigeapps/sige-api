use std::sync::Arc;

use domain::entities::{seclusion, seclusion_visit};
use sea_orm::*;

use crate::dtos::{
    prevention::seclusion::{
        visit::{AddSeclusionVisitDTO, GetSeclusionVisitDTO},
        CreateSeclusionDTO, GetSeclusionDTO, GetSeclusionWithVisitDTO, UpdateSeclusionExitDTO,
    },
    CommonQueryFilterDTO,
};

#[derive(Debug, Clone)]
pub struct SeclusionService {
    db: Arc<DatabaseConnection>,
}

impl SeclusionService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find(
        self,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetSeclusionWithVisitDTO>, DbErr> {
        let mut query = seclusion::Entity::find()
            .limit(filter.limit)
            .offset(filter.offset);

        if let Some(search) = filter.search {
            query = query.filter(
                Condition::any()
                    .add(seclusion::Column::Ci.contains(&search))
                    .add(seclusion::Column::ExitReason.contains(&search))
                    .add(seclusion::Column::LastName.contains(&search))
                    .add(seclusion::Column::FirstName.contains(&search))
                    .add(seclusion::Column::Observations.contains(&search)),
            )
        }

        let seclusions = query.all(&*self.db).await?;

        // Load all related visits using the loader pattern
        let visits = seclusions
            .load_many(seclusion_visit::Entity, &*self.db)
            .await?;

        // Combine results
        Ok(seclusions
            .into_iter()
            .zip(visits)
            .map(|(seclusion, visits)| GetSeclusionWithVisitDTO {
                seclusion: GetSeclusionDTO {
                    id: seclusion.id,
                    photo: seclusion.photo,
                    ci: seclusion.ci,
                    birthdate: seclusion.birthdate,
                    age: seclusion.age,
                    last_name: seclusion.last_name,
                    first_name: seclusion.first_name,
                    reason: seclusion.reason,
                    exit_reason: seclusion.exit_reason,
                    physical_state: seclusion.physical_state,
                    outfit: seclusion.outfit,
                    belongings: seclusion.belongings,
                    observations: seclusion.observations,
                    exit_at: seclusion.exit_at,
                },
                visits: visits
                    .into_iter()
                    .map(|v| GetSeclusionVisitDTO {
                        id: v.id,
                        seclusion_id: v.seclusion_id,
                        ci: v.ci,
                        last_name: v.last_name,
                        first_name: v.first_name,
                        relationship_id: v.relationship_id,
                        phone: v.phone,
                        date: v.date,
                        address: v.address,
                        reason: v.reason,
                    })
                    .collect(),
            })
            .collect())
    }

    pub async fn find_by_id(self, id: i32) -> Result<Option<GetSeclusionDTO>, DbErr> {
        seclusion::Entity::find_by_id(id)
            .into_partial_model::<GetSeclusionDTO>()
            .one(&*self.db)
            .await
    }

    pub async fn update_exit(self, id: i32, mut dto: UpdateSeclusionExitDTO) -> Result<i32, DbErr> {
        dto.id = id;

        dto.into_active_model().update(&*self.db).await?;

        Ok(id)
    }

    pub async fn create(self, dto: CreateSeclusionDTO) -> Result<i32, DbErr> {
        let seclusion_id = dto.into_active_model().insert(&*self.db).await?.id;

        Ok(seclusion_id)
    }

    pub async fn add_visit(self, id: i32, mut dto: AddSeclusionVisitDTO) -> Result<i32, DbErr> {
        dto.seclusion_id = id;

        let visit_id = dto.into_active_model().insert(&*self.db).await?.id;

        Ok(visit_id)
    }
}
