use crate::{
    api::ApiContext,
    dtos::{
        parking::issuance::{returns::FinalizeIssuance, IssuanceSummary, StartIssuance},
        CommonQueryFilterDTO,
    },
};
use domain::entities::{issuance, issuance_return};
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct IssuanceService {}

impl IssuanceService {
    pub async fn start(ctx: ApiContext, dto: StartIssuance) -> Result<i32, DbErr> {
        let id = dto.into_active_model().insert(&ctx.db).await?.id;

        Ok(id)
    }

    pub async fn finalize(ctx: ApiContext, dto: FinalizeIssuance) -> Result<i32, DbErr> {
        let id = dto.into_active_model().insert(&ctx.db).await?.issuance_id;

        Ok(id)
    }

    pub async fn find(
        ctx: ApiContext,
        opts: CommonQueryFilterDTO,
    ) -> Result<Vec<IssuanceSummary>, DbErr> {
        let mut query = issuance::Entity::find()
            .select_only()
            .column_as(issuance::Column::Id, "id")
            .column_as(issuance::Column::DateTime, "date_time")
            .column_as(issuance::Column::AssignanceDays, "assignance_days")
            .column_as(issuance_return::Column::ReturnedAt, "returned_at");

        if let Some(search) = &opts.search {
            query = query.filter(Condition::any().add(issuance::Column::Id.contains(search)))
        }

        if let Some(start) = &opts.from_date {
            query = query.filter(Condition::any().add(issuance::Column::DateTime.gte(*start)))
        }

        if let Some(end) = &opts.to_date {
            query = query.filter(Condition::any().add(issuance::Column::DateTime.lte(*end)))
        }

        if let Some(finalized) = &opts.finalized {
            match finalized {
                true => {
                    query = query
                        .filter(Condition::any().add(issuance_return::Column::Id.is_not_null()))
                }
                false => {
                    query =
                        query.filter(Condition::any().add(issuance_return::Column::Id.is_null()))
                }
            }
        }

        let pagination = &opts.into_pagination();

        let issuances = query
            .left_join(issuance_return::Entity)
            .limit(pagination.limit)
            .offset(pagination.offset)
            .into_model::<IssuanceSummary>()
            .all(&ctx.db)
            .await?;

        Ok(issuances)
    }
}
