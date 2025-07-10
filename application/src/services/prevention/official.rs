use domain::entities::{brigade, charge, hierarchy, official};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{
    api::ApiContext,
    auth::{FilterByClaims, HasBaseId, UserClaims, UserStamp},
    dtos::prevention::official::{CreateOfficialDTO, GetOfficialDTO},
    impl_filter_by_claims,
};

impl HasBaseId for official::ActiveModel {
    fn set_base_id(mut self, id: i32) -> Self {
        self.base_id = Set(id);

        self
    }
}

impl_filter_by_claims!(official, BaseId);

#[derive(Debug, Clone)]
pub struct OfficialService {}

impl OfficialService {
    pub async fn find(
        ctx: ApiContext,
        search: Option<String>,
        brigade_id: Option<i32>,
    ) -> Result<Vec<GetOfficialDTO>, DbErr> {
        let mut query = official::Entity::find()
            .filter_by_claims(ctx.claims)
            .left_join(brigade::Entity)
            .left_join(hierarchy::Entity)
            .left_join(charge::Entity);

        if let Some(search) = search {
            query = query.filter(official::Column::Ci.contains(search));
        }

        if let Some(brigade_id) = brigade_id {
            query = query.filter(official::Column::BrigadeId.eq(brigade_id));
        }

        query
            .into_partial_model::<GetOfficialDTO>()
            .all(&ctx.db)
            .await
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<Option<GetOfficialDTO>, DbErr> {
        let official = official::Entity::find_by_id(id)
            .filter_by_claims(ctx.claims)
            .select_only()
            .column_as(official::Column::Id, "id")
            .column_as(official::Column::Ci, "ci")
            .column_as(official::Column::Phone, "phone")
            .column_as(official::Column::LastName, "last_name")
            .column_as(official::Column::FirstName, "first_name")
            .column_as(brigade::Column::Name, "brigade")
            .column_as(hierarchy::Column::Name, "hierarchy")
            .column_as(charge::Column::Name, "charge")
            .join(JoinType::InnerJoin, brigade::Relation::Official.def())
            .join(JoinType::InnerJoin, hierarchy::Relation::Official.def())
            .join(JoinType::InnerJoin, charge::Relation::Official.def())
            .into_model::<GetOfficialDTO>()
            .one(&ctx.db)
            .await?;

        Ok(official)
    }

    pub async fn create(ctx: ApiContext, official: CreateOfficialDTO) -> Result<(), DbErr> {
        official::Entity::insert(official.into_active_model().stamp_user(ctx.claims))
            .exec(&ctx.db)
            .await?;

        Ok(())
    }
}
