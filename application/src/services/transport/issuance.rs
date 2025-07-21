use crate::{
    api::ApiContext,
    auth::HasBaseId,
    auth::{FilterByClaims, UserClaims, UserStamp},
    dtos::{
        transport::issuance::{
            returns::FinalizeTransportIssuance, StartTransportIssuance, TransportIssuanceSummary,
            TransportIssuanceView,
        },
        CommonQueryFilterDTO,
    },
    impl_filter_by_claims,
};
use domain::entities::{persona, transport, transport_issuance, transport_issuance_return};
use sea_orm::*;

#[derive(Debug, Clone)]
pub struct IssuanceService {}

impl HasBaseId for transport_issuance::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(transport_issuance, BaseId);

// TODO: SUPER IMPORTANTE: Refactorizar este servicio para poder usarlo en transporte

impl IssuanceService {
    pub async fn start(ctx: ApiContext, dto: StartTransportIssuance) -> Result<i32, DbErr> {
        let id = dto
            .into_active_model()
            .stamp_user(ctx.claims)
            .insert(&ctx.db)
            .await?
            .id;

        Ok(id)
    }

    pub async fn finalize(ctx: ApiContext, dto: FinalizeTransportIssuance) -> Result<i32, DbErr> {
        let id = dto.into_active_model().insert(&ctx.db).await?.issuance_id;

        Ok(id)
    }

    pub async fn find(
        ctx: ApiContext,
        opts: CommonQueryFilterDTO,
    ) -> Result<Vec<TransportIssuanceSummary>, DbErr> {
        let mut query = transport_issuance::Entity::find()
            .select_only()
            .column_as(transport_issuance::Column::Id, "id")
            .column_as(transport_issuance::Column::DateTime, "date_time")
            .column_as(
                transport_issuance::Column::AssignanceDays,
                "assignance_days",
            )
            .column_as(transport_issuance_return::Column::ReturnedAt, "returned_at");

        if let Some(search) = &opts.search {
            query =
                query.filter(Condition::any().add(transport_issuance::Column::Id.contains(search)))
        }

        if let Some(start) = &opts.from_date {
            query =
                query.filter(Condition::any().add(transport_issuance::Column::DateTime.gte(*start)))
        }

        if let Some(end) = &opts.to_date {
            query =
                query.filter(Condition::any().add(transport_issuance::Column::DateTime.lte(*end)))
        }

        if let Some(finalized) = &opts.finalized {
            match finalized {
                true => {
                    query = query.filter(
                        Condition::any().add(transport_issuance_return::Column::Id.is_not_null()),
                    )
                }
                false => {
                    query = query.filter(
                        Condition::any().add(transport_issuance_return::Column::Id.is_null()),
                    )
                }
            }
        }

        let pagination = &opts.into_pagination();

        let issuances = query
            .left_join(transport_issuance_return::Entity)
            .filter_by_claims(ctx.claims)
            .limit(pagination.limit)
            .offset(pagination.offset)
            .into_model::<TransportIssuanceSummary>()
            .all(&ctx.db)
            .await?;

        Ok(issuances)
    }

    pub async fn find_by_id(
        ctx: ApiContext,
        id: i32,
    ) -> Result<Option<TransportIssuanceView>, DbErr> {
        let query = transport_issuance::Entity::find_by_id(id)
            .column_as(transport_issuance::Column::Id, "id")
            .column_as(transport_issuance::Column::DateTime, "date_time")
            .column_as(
                transport_issuance::Column::AssignanceDays,
                "assignance_days",
            )
            .column_as(transport_issuance::Column::Type, "type")
            .column_as(transport_issuance_return::Column::ReturnedAt, "returned_at");

        let issuance = query
            .left_join(transport_issuance_return::Entity)
            .left_join(persona::Entity)
            .left_join(transport::Entity)
            .filter_by_claims(ctx.claims)
            .into_partial_model::<TransportIssuanceView>()
            .one(&ctx.db)
            .await?;

        Ok(issuance)
    }
}
