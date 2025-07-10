use crate::Result;
use application::{
    dtos::lookup::CreateBasicLookUpDTO, services::prevention::lookup::LookupService,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use domain::entities::{
    band, base, brand, brigade, charge, division, document_type, family_relationship, hierarchy,
    institution, municipality, novelty, organism, parish, persona_state, profession, role,
    seclusion_statuses, state, status_condition, transport_statuses, transport_type, vehicle_model,
};
use tracing::debug;

use application::api::ApiContext;

// TODO: Refactorizar para que no se repita el codigo

pub async fn get_brigades(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let brigades =
        LookupService::find::<brigade::Entity, brigade::Model, brigade::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&brigades));
    Ok((StatusCode::OK, Json(brigades)).into_response())
}

pub async fn create_brigade(
    Extension(ctx): Extension<ApiContext>,
    Json(brigade): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = brigade::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(brigade.name),
    };

    LookupService::create::<brigade::Entity, brigade::Model, brigade::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_organisms(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let organisms =
        LookupService::find::<organism::Entity, organism::Model, organism::ActiveModel>(ctx)
            .await?;

    debug!("{:?}", Json(&organisms));
    Ok((StatusCode::OK, Json(organisms)).into_response())
}

pub async fn create_organism(
    Extension(ctx): Extension<ApiContext>,
    Json(organism): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = organism::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(organism.name),
        phone: sea_orm::Set(String::new()), // Default empty string for phone
    };

    LookupService::create::<organism::Entity, organism::Model, organism::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_divisions(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let divisions =
        LookupService::find::<division::Entity, division::Model, division::ActiveModel>(ctx)
            .await?;

    debug!("{:?}", Json(&divisions));
    Ok((StatusCode::OK, Json(divisions)).into_response())
}

pub async fn create_division(
    Extension(ctx): Extension<ApiContext>,
    Json(division): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = division::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(division.name),
        state: sea_orm::Set(division.state.unwrap_or_default()),
    };

    LookupService::create::<division::Entity, division::Model, division::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_charges(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let charges =
        LookupService::find::<charge::Entity, charge::Model, charge::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&charges));
    Ok((StatusCode::OK, Json(charges)).into_response())
}

pub async fn create_charge(
    Extension(ctx): Extension<ApiContext>,
    Json(charge): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = charge::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(charge.name),
    };

    LookupService::create::<charge::Entity, charge::Model, charge::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_hierarchies(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let hierarchies =
        LookupService::find::<hierarchy::Entity, hierarchy::Model, hierarchy::ActiveModel>(ctx)
            .await?;

    debug!("{:?}", Json(&hierarchies));
    Ok((StatusCode::OK, Json(hierarchies)).into_response())
}

pub async fn create_hierarchy(
    Extension(ctx): Extension<ApiContext>,
    Json(hierarchy): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = hierarchy::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(hierarchy.name),
    };

    LookupService::create::<hierarchy::Entity, hierarchy::Model, hierarchy::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_states(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let states =
        LookupService::find::<state::Entity, state::Model, state::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&states));
    Ok((StatusCode::OK, Json(states)).into_response())
}

pub async fn create_state(
    Extension(ctx): Extension<ApiContext>,
    Json(state): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = state::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(state.name),
    };

    LookupService::create::<state::Entity, state::Model, state::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_municipalities(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let municipalities = LookupService::find::<
        municipality::Entity,
        municipality::Model,
        municipality::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&municipalities));
    Ok((StatusCode::OK, Json(municipalities)).into_response())
}

pub async fn create_municipality(
    Extension(ctx): Extension<ApiContext>,
    Json(municipality): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = municipality::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(municipality.name),
        state: sea_orm::Set(municipality.state.unwrap_or_default()),
    };

    LookupService::create::<municipality::Entity, municipality::Model, municipality::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_transport_types(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let transport_types = LookupService::find::<
        transport_type::Entity,
        transport_type::Model,
        transport_type::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&transport_types));
    Ok((StatusCode::OK, Json(transport_types)).into_response())
}

pub async fn create_transport_type(
    Extension(ctx): Extension<ApiContext>,
    Json(transport_type): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = transport_type::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(transport_type.name),
    };

    LookupService::create::<
        transport_type::Entity,
        transport_type::Model,
        transport_type::ActiveModel,
    >(ctx, active_model)
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_transport_statuses(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let transport_statuses = LookupService::find::<
        transport_statuses::Entity,
        transport_statuses::Model,
        transport_statuses::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&transport_statuses));
    Ok((StatusCode::OK, Json(transport_statuses)).into_response())
}

pub async fn create_transport_status(
    Extension(ctx): Extension<ApiContext>,
    Json(transport_status): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = transport_statuses::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(transport_status.name),
    };

    LookupService::create::<
        transport_statuses::Entity,
        transport_statuses::Model,
        transport_statuses::ActiveModel,
    >(ctx, active_model)
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_brands(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let brands =
        LookupService::find::<brand::Entity, brand::Model, brand::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&brands));
    Ok((StatusCode::OK, Json(brands)).into_response())
}

pub async fn create_brand(
    Extension(ctx): Extension<ApiContext>,
    Json(brand): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = brand::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(brand.name),
    };

    LookupService::create::<brand::Entity, brand::Model, brand::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_vehicle_models(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let vehicle_models = LookupService::find::<
        vehicle_model::Entity,
        vehicle_model::Model,
        vehicle_model::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&vehicle_models));
    Ok((StatusCode::OK, Json(vehicle_models)).into_response())
}

pub async fn create_vehicle_model(
    Extension(ctx): Extension<ApiContext>,
    Json(vehicle_model): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    if vehicle_model.brand.is_none() {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, "no brand id was provided").into_response());
    };

    let active_model = vehicle_model::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(vehicle_model.name),
        brand: sea_orm::Set(vehicle_model.brand.unwrap_or_default()),
    };

    LookupService::create::<vehicle_model::Entity, vehicle_model::Model, vehicle_model::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_parish(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let parishes =
        LookupService::find::<parish::Entity, parish::Model, parish::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&parishes));
    Ok((StatusCode::OK, Json(parishes)).into_response())
}

pub async fn create_parish(
    Extension(ctx): Extension<ApiContext>,
    Json(parish): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = parish::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(parish.name),
    };

    LookupService::create::<parish::Entity, parish::Model, parish::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_seclusion_statuses(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let seclusion_statuses = LookupService::find::<
        seclusion_statuses::Entity,
        seclusion_statuses::Model,
        seclusion_statuses::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&seclusion_statuses));
    Ok((StatusCode::OK, Json(seclusion_statuses)).into_response())
}

pub async fn create_seclusion_statuses(
    Extension(ctx): Extension<ApiContext>,
    Json(seclusion_status): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = seclusion_statuses::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(seclusion_status.name),
    };

    LookupService::create::<
        seclusion_statuses::Entity,
        seclusion_statuses::Model,
        seclusion_statuses::ActiveModel,
    >(ctx, active_model)
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_family_relationships(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let family_relationships = LookupService::find::<
        family_relationship::Entity,
        family_relationship::Model,
        family_relationship::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&family_relationships));
    Ok((StatusCode::OK, Json(family_relationships)).into_response())
}

pub async fn create_family_relationships(
    Extension(ctx): Extension<ApiContext>,
    Json(family_relationship): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = family_relationship::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(family_relationship.name),
    };

    LookupService::create::<
        family_relationship::Entity,
        family_relationship::Model,
        family_relationship::ActiveModel,
    >(ctx, active_model)
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_novelties(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let novelties =
        LookupService::find::<novelty::Entity, novelty::Model, novelty::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&novelties));
    Ok((StatusCode::OK, Json(novelties)).into_response())
}

pub async fn create_novelty(
    Extension(ctx): Extension<ApiContext>,
    Json(novelty): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = novelty::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(novelty.name),
        format: sea_orm::Set(novelty.format.unwrap_or_default()),
    };

    LookupService::create::<novelty::Entity, novelty::Model, novelty::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_status_conditions(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let status_conditions = LookupService::find::<
        status_condition::Entity,
        status_condition::Model,
        status_condition::ActiveModel,
    >(ctx)
    .await?;

    debug!("{:?}", Json(&status_conditions));
    Ok((StatusCode::OK, Json(status_conditions)).into_response())
}

pub async fn create_band(
    Extension(ctx): Extension<ApiContext>,
    Json(band): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = band::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(band.name),
    };

    LookupService::create::<band::Entity, band::Model, band::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_bands(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let bands = LookupService::find::<band::Entity, band::Model, band::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&bands));
    Ok((StatusCode::OK, Json(bands)).into_response())
}

pub async fn create_institution(
    Extension(ctx): Extension<ApiContext>,
    Json(institution): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = institution::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(institution.name),
    };

    LookupService::create::<institution::Entity, institution::Model, institution::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_institutions(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let institutions =
        LookupService::find::<institution::Entity, institution::Model, institution::ActiveModel>(
            ctx,
        )
        .await?;

    debug!("{:?}", Json(&institutions));
    Ok((StatusCode::OK, Json(institutions)).into_response())
}

pub async fn create_profession(
    Extension(ctx): Extension<ApiContext>,
    Json(profession): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = profession::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(profession.name),
    };

    LookupService::create::<profession::Entity, profession::Model, profession::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_professions(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let professions =
        LookupService::find::<profession::Entity, profession::Model, profession::ActiveModel>(ctx)
            .await?;

    debug!("{:?}", Json(&professions));
    Ok((StatusCode::OK, Json(professions)).into_response())
}

pub async fn get_bases(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let bases = LookupService::find::<base::Entity, base::Model, base::ActiveModel>(ctx).await?;

    debug!("roles: {:?}", Json(&bases));
    Ok((StatusCode::OK, Json(bases)).into_response())
}

pub async fn create_base(
    Extension(ctx): Extension<ApiContext>,
    Json(base): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = base::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(base.name),
    };

    LookupService::create::<base::Entity, base::Model, base::ActiveModel>(ctx, active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_roles(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let roles = LookupService::find::<role::Entity, role::Model, role::ActiveModel>(ctx).await?;

    debug!("{:?}", Json(&roles));
    Ok((StatusCode::OK, Json(roles)).into_response())
}

pub async fn get_persona_states(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let persona_states = LookupService::find::<
        persona_state::Entity,
        persona_state::Model,
        persona_state::ActiveModel,
    >(ctx)
    .await?;

    debug!("persona_states: {:?}", Json(&persona_states));
    Ok((StatusCode::OK, Json(persona_states)).into_response())
}

pub async fn create_persona_state(
    Extension(ctx): Extension<ApiContext>,
    Json(persona_state): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = persona_state::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(persona_state.name),
    };

    LookupService::create::<persona_state::Entity, persona_state::Model, persona_state::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_document_types(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let document_types = LookupService::find::<
        document_type::Entity,
        document_type::Model,
        document_type::ActiveModel,
    >(ctx)
    .await?;

    debug!("persona_states: {:?}", Json(&document_types));
    Ok((StatusCode::OK, Json(document_types)).into_response())
}

pub async fn create_document_type(
    Extension(ctx): Extension<ApiContext>,
    Json(document_type): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = document_type::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(document_type.name),
    };

    LookupService::create::<document_type::Entity, document_type::Model, document_type::ActiveModel>(
        ctx,
        active_model,
    )
    .await?;

    Ok(StatusCode::CREATED.into_response())
}
