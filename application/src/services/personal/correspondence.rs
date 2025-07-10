use domain::entities::{correspondence, correspondence_document, document_type};
use sea_orm::*;

use crate::{
    api::ApiContext,
    dtos::{
        personal::correspondence::{
            document::CorrespondenceDocumentResponse, CorrespondenceResponse,
            CorrespondenceSummary, CreateCorrespondenceRequest,
        },
        CommonQueryFilterDTO, PaginationDTO,
    },
};

#[derive(Debug, Clone)]
pub struct CorrespondenceService {}

impl CorrespondenceService {
    pub async fn create(ctx: ApiContext, dto: CreateCorrespondenceRequest) -> Result<i32, DbErr> {
        let transaction = ctx.db.begin().await?;

        let id = dto.dto.into_active_model().insert(&transaction).await?.id;

        async {
            for mut document in dto.documents {
                document.correspondence_id = id;

                document.into_active_model().insert(&transaction).await?;
            }
            Ok::<(), DbErr>(())
        }
        .await?;

        transaction.commit().await?;

        Ok(id)
    }

    pub async fn find(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<Vec<CorrespondenceSummary>, DbErr> {
        let mut query = correspondence::Entity::find().left_join(correspondence_document::Entity);

        if let Some(search) = &filter.search {
            query = query.filter(correspondence::Column::Type.contains(search));
        }
        if let Some(date) = &filter.from_date {
            query = query.filter(correspondence::Column::DateTime.gte(*date));
        }
        if let Some(date) = &filter.to_date {
            query = query.filter(correspondence::Column::DateTime.lte(*date));
        }

        query = query.column_as(
            correspondence_document::Column::Id.count(),
            "document_count",
        );

        let pagination = &filter.into_pagination();

        query
            .limit(pagination.limit)
            .offset(pagination.offset)
            .group_by(correspondence::Column::Id)
            .into_partial_model::<CorrespondenceSummary>()
            .all(&ctx.db)
            .await
    }

    pub async fn get_pagination(
        ctx: ApiContext,
        filter: CommonQueryFilterDTO,
    ) -> Result<PaginationDTO, DbErr> {
        let mut query = correspondence::Entity::find();

        if let Some(search) = &filter.search {
            query = query.filter(correspondence::Column::Type.contains(search));
        }
        if let Some(date) = &filter.from_date {
            query = query.filter(correspondence::Column::DateTime.gte(*date));
        }
        if let Some(date) = &filter.to_date {
            query = query.filter(correspondence::Column::DateTime.lte(*date));
        }

        let pagination = &filter.into_pagination();

        let paginator = query.paginate(&ctx.db, pagination.limit);

        let total_count = paginator.num_items().await?;
        let page_count = paginator.num_pages().await?;

        Ok(PaginationDTO {
            page: pagination.page,
            limit: pagination.limit,
            page_count,
            total_count,
            offset: pagination.offset,
        })
    }

    pub async fn find_by_id(
        ctx: ApiContext,
        id: i32,
    ) -> Result<Option<CorrespondenceResponse>, DbErr> {
        let correspondence = correspondence::Entity::find_by_id(id)
            .column_as(
                correspondence_document::Column::Id.count(),
                "document_count",
            )
            .into_partial_model::<CorrespondenceSummary>()
            .one(&ctx.db)
            .await?;

        let Some(correspondence) = correspondence else {
            return Ok(None);
        };

        let documents = correspondence_document::Entity::find()
            .left_join(document_type::Entity)
            .filter(correspondence_document::Column::CorrespondenceId.eq(correspondence.id))
            .into_partial_model::<CorrespondenceDocumentResponse>()
            .all(&ctx.db)
            .await?;

        Ok(Some(CorrespondenceResponse {
            dto: correspondence,
            documents,
        }))
    }
}
