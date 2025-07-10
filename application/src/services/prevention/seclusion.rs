use domain::entities::{family_relationship, seclusion, seclusion_visit};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{FilterByClaims, HasBaseId, UserClaims, UserStamp},
    dtos::{
        prevention::seclusion::{
            visit::{AddSeclusionVisitDTO, GetSeclusionVisitDTO},
            CreateSeclusionDTO, GetSeclusionDTO, GetSeclusionWithVisitDTO, UpdateSeclusionExitDTO,
        },
        CommonQueryFilterDTO, PaginationDTO,
    },
    impl_filter_by_claims,
};

impl HasBaseId for seclusion::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(seclusion, BaseId);

#[derive(Debug, Clone)]
pub struct SeclusionService {}

impl SeclusionService {
    // TODO: THIS FUNCTION REALLY NEEDS TO RETURN AN REAL OPTION
    pub async fn find_by_id(
        ctx: ApiContext,
        id: i32,
    ) -> Result<Option<GetSeclusionWithVisitDTO>, DbErr> {
        let query = seclusion::Entity::find_by_id(id).filter_by_claims(ctx.claims);

        let seclusion = query
            .one(&ctx.db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("Seclusion with id {} not found", id)))?;

        let visits = seclusion
            .find_related(seclusion_visit::Entity)
            .left_join(family_relationship::Entity)
            .into_partial_model::<GetSeclusionVisitDTO>()
            .all(&ctx.db)
            .await?;

        let seclusion = GetSeclusionDTO::from(seclusion);

        Ok(Some(GetSeclusionWithVisitDTO { seclusion, visits }))
    }

    pub async fn find(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<GetSeclusionDTO>, DbErr> {
        let mut query = seclusion::Entity::find().filter_by_claims(ctx.claims);

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
            .all(&ctx.db)
            .await
    }

    pub async fn get_pagination(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = seclusion::Entity::find().filter_by_claims(ctx.claims);

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

        let paginator = query.paginate(&ctx.db, pagination.limit);

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

    pub async fn update_exit(
        ctx: ApiContext,
        id: i32,
        mut dto: UpdateSeclusionExitDTO,
    ) -> Result<i32, DbErr> {
        dto.id = id;

        dto.into_active_model().update(&ctx.db).await?;

        Ok(id)
    }

    pub async fn create(ctx: ApiContext, dto: CreateSeclusionDTO) -> Result<i32, DbErr> {
        let seclusion_id = dto
            .into_active_model()
            .stamp_user(ctx.claims)
            .insert(&ctx.db)
            .await?
            .id;

        Ok(seclusion_id)
    }

    pub async fn add_visit(
        ctx: ApiContext,
        id: i32,
        mut dto: AddSeclusionVisitDTO,
    ) -> Result<i32, DbErr> {
        dto.seclusion_id = id;

        let visit_id = dto.into_active_model().insert(&ctx.db).await?.id;

        Ok(visit_id)
    }
}
