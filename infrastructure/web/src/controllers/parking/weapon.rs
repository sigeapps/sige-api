use crate::{tags::WEAPON_TAG, Result};
use application::{
    api::ApiContext,
    dtos::{
        parking::weapon::{WeaponCreate, WeaponView},
        CommonQueryFilterDTO,
    },
    services::parking::weapon::WeaponService,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WeaponBody {
    weapons: Vec<WeaponView>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SingleWeaponBody {
    weapon: WeaponView,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateWeaponResponse {
    weapon_id: i32,
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
            ("id" = i32, Path, description = "ID del arma")
    ),
    tag = WEAPON_TAG,
    responses(
        (status = 200, description = "Arma individual", body = SingleWeaponBody),
        (status = 404, description = "El arma no existe"),
    ),
)]
pub async fn get_weapon_by_id(
    Path(id): Path<i32>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Response> {
    let weapon = WeaponService::find_by_id(ctx, id).await?;

    if weapon.is_none() {
        return Ok((StatusCode::NOT_FOUND, "Weapon not found").into_response());
    }

    Ok((
        StatusCode::OK,
        Json(SingleWeaponBody {
            weapon: weapon.unwrap(),
        }),
    )
        .into_response())
}

#[utoipa::path(
    get,
    path = "",
    tag = WEAPON_TAG,
    responses(
        (status = 200, description = "Listado de armas", body = WeaponBody),
    ),
 params(
            CommonQueryFilterDTO,
        )
)]
pub async fn get_weapons(
    Query(query): Query<CommonQueryFilterDTO>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Response> {
    let weapons = WeaponService::find(ctx, query).await?;

    Ok((StatusCode::OK, Json(WeaponBody { weapons })).into_response())
}

#[utoipa::path(
    post,
    path = "",
    tag = WEAPON_TAG,
    request_body = WeaponCreate,
    responses(
        (status = 200, description = "La arma fue creada excitosamente", body = CreateWeaponResponse)
    )
)]
pub async fn create_weapon(
    Extension(ctx): Extension<ApiContext>,
    Json(weapon): Json<WeaponCreate>,
) -> Result<Response> {
    let weapon_id = WeaponService::create(ctx, weapon).await?;

    Ok((StatusCode::OK, Json(CreateWeaponResponse { weapon_id })).into_response())
}
