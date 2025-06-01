use std::sync::Arc;

use domain::entities::{family_relationship, seclusion, seclusion_visit};
use sea_orm::*;

use crate::dtos::{
    prevention::seclusion::{
        visit::{AddSeclusionVisitDTO, GetSeclusionVisitDTO},
        CreateSeclusionDTO, GetSeclusionDTO, GetSeclusionWithVisitDTO, UpdateSeclusionExitDTO,
    },
    CommonQueryFilterDTO, PaginationDTO,
};

#[derive(Debug, Clone)]
pub struct SeclusionService {
    db: Arc<DatabaseConnection>,
}

impl SeclusionService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    // TODO: THIS FUNCTION REALLY NEEDS TO RETURN AN REAL OPTION
    pub async fn find_by_id(self, id: i32) -> Result<Option<GetSeclusionWithVisitDTO>, DbErr> {
        let query = seclusion::Entity::find_by_id(id);

        let seclusion = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("Seclusion with id {} not found", id)))?;

        let visits = seclusion
            .find_related(seclusion_visit::Entity)
            .left_join(family_relationship::Entity)
            .into_partial_model::<GetSeclusionVisitDTO>()
            .all(&*self.db)
            .await?;

        let seclusion = GetSeclusionDTO::from(seclusion);

        Ok(Some(GetSeclusionWithVisitDTO { seclusion, visits }))
    }

    pub async fn find(self, filter: CommonQueryFilterDTO) -> Result<Vec<GetSeclusionDTO>, DbErr> {
        let mut query = seclusion::Entity::find();

        if let Some(search) = &filter.search {
            query = query.filter(
                Condition::all()
                    .add(seclusion::Column::Reason.contains(search))
                    .add(seclusion::Column::Belongings.contains(search))
                    .add(seclusion::Column::ExitReason.contains(search))
                    .add(seclusion::Column::Ci.contains(search))
                    .add(seclusion::Column::FirstName.contains(search))
                    .add(seclusion::Column::LastName.contains(search)),
            )
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(seclusion::Column::CreatedAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(seclusion::Column::CreatedAt.lte(*to_date));
        }

        let pagination = &filter.into_pagination();

        query
            .limit(pagination.limit)
            .offset(pagination.offset)
            .into_partial_model::<GetSeclusionDTO>()
            .all(&*self.db)
            .await
    }

    pub async fn get_pagination(
        self,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = seclusion::Entity::find();

        if let Some(search) = &filter.search {
            query = query.filter(
                Condition::all()
                    .add(seclusion::Column::Reason.contains(search))
                    .add(seclusion::Column::Belongings.contains(search))
                    .add(seclusion::Column::ExitReason.contains(search))
                    .add(seclusion::Column::Ci.contains(search))
                    .add(seclusion::Column::FirstName.contains(search))
                    .add(seclusion::Column::LastName.contains(search)),
            )
        }

        if let Some(from_date) = &filter.from_date {
            query = query.filter(seclusion::Column::CreatedAt.gte(*from_date));
        }

        if let Some(to_date) = &filter.to_date {
            query = query.filter(seclusion::Column::CreatedAt.lte(*to_date));
        }

        let pagination = &filter.into_pagination();

        let paginator = query.paginate(&*self.db, pagination.limit);

        let items = paginator.num_items().await?;
        let pages = paginator.num_pages().await?;

        Ok(PaginationDTO {
            offset: pagination.offset,
            page_count: pages,
            total_count: items,
            page: pagination.page,
            limit: pagination.limit,
        })
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
