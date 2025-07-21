use crate::{
    api::ApiContext,
    auth::HasBaseId,
    auth::{FilterByClaims, UserClaims, UserStamp},
    dtos::{
        parking::issuance::{
            returns::FinalizeIssuance, IssuanceSummary, IssuanceView, StartIssuance,
        },
        CommonQueryFilterDTO,
    },
    impl_filter_by_claims,
};
use domain::entities::{issuance, issuance_return, persona, weapon};
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct IssuanceService {}

impl HasBaseId for issuance::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(issuance, BaseId);

// TODO: SUPER IMPORTANTE: Refactorizar este servicio para poder usarlo en transporte

impl IssuanceService {
    pub async fn start(ctx: ApiContext, dto: StartIssuance) -> Result<i32, DbErr> {
        let id = dto
            .into_active_model()
            .stamp_user(ctx.claims)
            .insert(&ctx.db)
            .await?
            .id;

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
            .filter_by_claims(ctx.claims)
            .limit(pagination.limit)
            .offset(pagination.offset)
            .into_model::<IssuanceSummary>()
            .all(&ctx.db)
            .await?;

        Ok(issuances)
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<Option<IssuanceView>, DbErr> {
        let query = issuance::Entity::find_by_id(id)
            .column_as(issuance::Column::Id, "id")
            .column_as(issuance::Column::DateTime, "date_time")
            .column_as(issuance::Column::AssignanceDays, "assignance_days")
            .column_as(issuance::Column::Type, "type")
            .column_as(issuance_return::Column::ReturnedAt, "returned_at");

        let issuance = query
            .left_join(issuance_return::Entity)
            .left_join(persona::Entity)
            .left_join(weapon::Entity)
            .join(JoinType::LeftJoin, weapon::Relation::WeaponModel.def())
            .join(JoinType::LeftJoin, weapon::Relation::WeaponType.def())
            .filter_by_claims(ctx.claims)
            .into_partial_model::<IssuanceView>()
            .one(&ctx.db)
            .await?;

        Ok(issuance)
    }
}
