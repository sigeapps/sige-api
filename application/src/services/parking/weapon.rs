use domain::entities::{weapon, weapon_model, weapon_type};
use sea_orm::*;

use crate::{
    api::ApiContext,
    dtos::parking::weapon::{WeaponCreate, WeaponView},
};

pub struct WeaponService {}

impl WeaponService {
    pub async fn create(ctx: ApiContext, dto: WeaponCreate) -> Result<i32, DbErr> {
        let id = dto.into_active_model().insert(&ctx.db).await?.id;

        Ok(id)
    }

    pub async fn find_by_id(ctx: ApiContext, id: i32) -> Result<Option<WeaponView>, DbErr> {
        weapon::Entity::find_by_id(id)
            .column_as(weapon_type::Column::Name, "type")
            .column_as(weapon_model::Column::Name, "model")
            .left_join(weapon_type::Entity)
            .left_join(weapon_model::Entity)
            .into_model::<WeaponView>()
            .one(&ctx.db)
            .await
    }
}
