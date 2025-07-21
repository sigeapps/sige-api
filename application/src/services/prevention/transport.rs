use domain::entities::{brand, transport, transport_statuses, transport_type, vehicle_model};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{FilterByClaims, HasBaseId, UserClaims, UserStamp},
    dtos::prevention::transport::{CreateTransportDTO, GetTransportDTO},
    impl_filter_by_claims,
};

impl HasBaseId for transport::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(transport, BaseId);

#[derive(Debug, Clone)]
pub struct TransportService {}

impl TransportService {
    pub async fn find(
        ctx: ApiContext,
        search: Option<String>,
    ) -> Result<Vec<GetTransportDTO>, DbErr> {
        let mut query = transport::Entity::find()
            .filter_by_claims(ctx.claims)
            .left_join(transport_type::Entity)
            .left_join(brand::Entity)
            .left_join(vehicle_model::Entity)
            .left_join(transport_statuses::Entity)
            .order_by_desc(transport::Column::Id);

        if let Some(search) = search {
            query = query.filter(transport::Column::Details.contains(search));
        }

        query
            .into_partial_model::<GetTransportDTO>()
            .all(&ctx.db)
            .await
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<Option<GetTransportDTO>, DbErr> {
        transport::Entity::find_by_id(id)
            .filter_by_claims(ctx.claims)
            .into_model::<GetTransportDTO>()
            .one(&ctx.db)
            .await
    }

    pub async fn create(ctx: ApiContext, transport: CreateTransportDTO) -> Result<(), DbErr> {
        transport::Entity::insert(transport.into_active_model().stamp_user(ctx.claims))
            .exec(&ctx.db)
            .await?;

        Ok(())
    }
}
