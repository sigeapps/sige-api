use crate::Result;
use application::dtos::lookup::CreateBasicLookUpDTO;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::entities::{
    band, base, brand, brigade, charge, division, family_relationship, hierarchy, institution,
    municipality, novelty, organism, parish, profession, seclusion_statuses, state,
    status_condition, transport_statuses, transport_type, vehicle_model,
};
use tracing::debug;

use crate::state::AppState;

pub async fn get_brigades(State(app_state): State<AppState>) -> Result<Response> {
    let brigades = app_state
        .lookup_service
        .find::<brigade::Entity, brigade::Model, brigade::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&brigades));
    Ok((StatusCode::OK, Json(brigades)).into_response())
}

pub async fn create_brigade(
    State(app_state): State<AppState>,
    Json(brigade): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = brigade::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(brigade.name),
    };

    app_state
        .lookup_service
        .create::<brigade::Entity, brigade::Model, brigade::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_organisms(State(app_state): State<AppState>) -> Result<Response> {
    let organisms = app_state
        .lookup_service
        .find::<organism::Entity, organism::Model, organism::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&organisms));
    Ok((StatusCode::OK, Json(organisms)).into_response())
}

pub async fn create_organism(
    State(app_state): State<AppState>,
    Json(organism): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = organism::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(organism.name),
        phone: sea_orm::Set(String::new()), // Default empty string for phone
    };

    app_state
        .lookup_service
        .create::<organism::Entity, organism::Model, organism::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn create_division(
    State(app_state): State<AppState>,
    Json(division): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = division::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(division.name),
        state: sea_orm::Set(division.state.unwrap_or_default()),
    };

    app_state
        .lookup_service
        .create::<division::Entity, division::Model, division::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_charges(State(app_state): State<AppState>) -> Result<Response> {
    let charges = app_state
        .lookup_service
        .find::<charge::Entity, charge::Model, charge::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&charges));
    Ok((StatusCode::OK, Json(charges)).into_response())
}

pub async fn create_charge(
    State(app_state): State<AppState>,
    Json(charge): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = charge::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(charge.name),
    };

    app_state
        .lookup_service
        .create::<charge::Entity, charge::Model, charge::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_hierarchies(State(app_state): State<AppState>) -> Result<Response> {
    let hierarchies = app_state
        .lookup_service
        .find::<hierarchy::Entity, hierarchy::Model, hierarchy::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&hierarchies));
    Ok((StatusCode::OK, Json(hierarchies)).into_response())
}

pub async fn create_hierarchy(
    State(app_state): State<AppState>,
    Json(hierarchy): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = hierarchy::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(hierarchy.name),
    };

    app_state
        .lookup_service
        .create::<hierarchy::Entity, hierarchy::Model, hierarchy::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_states(State(app_state): State<AppState>) -> Result<Response> {
    let states = app_state
        .lookup_service
        .find::<state::Entity, state::Model, state::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&states));
    Ok((StatusCode::OK, Json(states)).into_response())
}

pub async fn create_state(
    State(app_state): State<AppState>,
    Json(state): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = state::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(state.name),
    };

    app_state
        .lookup_service
        .create::<state::Entity, state::Model, state::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_municipalities(State(app_state): State<AppState>) -> Result<Response> {
    let municipalities = app_state
        .lookup_service
        .find::<municipality::Entity, municipality::Model, municipality::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&municipalities));
    Ok((StatusCode::OK, Json(municipalities)).into_response())
}

pub async fn create_municipality(
    State(app_state): State<AppState>,
    Json(municipality): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = municipality::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(municipality.name),
        state: sea_orm::Set(municipality.state.unwrap_or_default()),
    };

    app_state
        .lookup_service
        .create::<municipality::Entity, municipality::Model, municipality::ActiveModel>(
            active_model,
        )
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_transport_types(State(app_state): State<AppState>) -> Result<Response> {
    let transport_types = app_state
        .lookup_service
        .find::<transport_type::Entity, transport_type::Model, transport_type::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&transport_types));
    Ok((StatusCode::OK, Json(transport_types)).into_response())
}

pub async fn create_transport_type(
    State(app_state): State<AppState>,
    Json(transport_type): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = transport_type::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(transport_type.name),
    };

    app_state
        .lookup_service
        .create::<transport_type::Entity, transport_type::Model, transport_type::ActiveModel>(
            active_model,
        )
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_transport_statuses(State(app_state): State<AppState>) -> Result<Response> {
    let transport_statuses = app_state
        .lookup_service
        .find::<transport_statuses::Entity, transport_statuses::Model, transport_statuses::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&transport_statuses));
    Ok((StatusCode::OK, Json(transport_statuses)).into_response())
}

pub async fn create_transport_status(
    State(app_state): State<AppState>,
    Json(transport_status): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = transport_statuses::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(transport_status.name),
    };

    app_state
        .lookup_service
        .create::<transport_statuses::Entity, transport_statuses::Model, transport_statuses::ActiveModel>(
            active_model,
        )
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_brands(State(app_state): State<AppState>) -> Result<Response> {
    let brands = app_state
        .lookup_service
        .find::<brand::Entity, brand::Model, brand::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&brands));
    Ok((StatusCode::OK, Json(brands)).into_response())
}

pub async fn create_brand(
    State(app_state): State<AppState>,
    Json(brand): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = brand::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(brand.name),
    };

    app_state
        .lookup_service
        .create::<brand::Entity, brand::Model, brand::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_vehicle_models(State(app_state): State<AppState>) -> Result<Response> {
    let vehicle_models = app_state.lookup_service.find_vehicle_models().await?;

    debug!("{:?}", Json(&vehicle_models));
    Ok((StatusCode::OK, Json(vehicle_models)).into_response())
}

pub async fn create_vehicle_model(
    State(app_state): State<AppState>,
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

    app_state
        .lookup_service
        .create::<vehicle_model::Entity, vehicle_model::Model, vehicle_model::ActiveModel>(
            active_model,
        )
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_parish(State(app_state): State<AppState>) -> Result<Response> {
    let vehicle_models = app_state
        .lookup_service
        .find::<parish::Entity, parish::Model, parish::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&vehicle_models));
    Ok((StatusCode::OK, Json(vehicle_models)).into_response())
}

pub async fn create_parish(
    State(app_state): State<AppState>,
    Json(vehicle_model): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = parish::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(vehicle_model.name),
    };

    app_state
        .lookup_service
        .create::<parish::Entity, parish::Model, parish::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_seclusion_statuses(State(app_state): State<AppState>) -> Result<Response> {
    let vehicle_models = app_state
        .lookup_service
        .find::<seclusion_statuses::Entity, seclusion_statuses::Model, seclusion_statuses::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&vehicle_models));
    Ok((StatusCode::OK, Json(vehicle_models)).into_response())
}

pub async fn create_seclusion_statuses(
    State(app_state): State<AppState>,
    Json(vehicle_model): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = seclusion_statuses::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(vehicle_model.name),
    };

    app_state
        .lookup_service
        .create::<seclusion_statuses::Entity, seclusion_statuses::Model, seclusion_statuses::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_family_relationships(State(app_state): State<AppState>) -> Result<Response> {
    let vehicle_models = app_state
        .lookup_service
        .find::<family_relationship::Entity, family_relationship::Model, family_relationship::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&vehicle_models));
    Ok((StatusCode::OK, Json(vehicle_models)).into_response())
}

pub async fn create_family_relationships(
    State(app_state): State<AppState>,
    Json(vehicle_model): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = family_relationship::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(vehicle_model.name),
    };

    app_state
        .lookup_service
        .create::<family_relationship::Entity, family_relationship::Model, family_relationship::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_novelties(State(app_state): State<AppState>) -> Result<Response> {
    let novelties = app_state
        .lookup_service
        .find::<novelty::Entity, novelty::Model, novelty::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&novelties));
    Ok((StatusCode::OK, Json(novelties)).into_response())
}

pub async fn create_novelty(
    State(app_state): State<AppState>,
    Json(novelty): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = novelty::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(novelty.name),
        format: sea_orm::Set(novelty.format.unwrap_or_default()),
    };

    app_state
        .lookup_service
        .create::<novelty::Entity, novelty::Model, novelty::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_status_conditions(State(app_state): State<AppState>) -> Result<Response> {
    let novelties = app_state
        .lookup_service
        .find::<status_condition::Entity, status_condition::Model, status_condition::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&novelties));
    Ok((StatusCode::OK, Json(novelties)).into_response())
}

pub async fn create_band(
    State(app_state): State<AppState>,
    Json(novelty): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = band::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(novelty.name),
    };

    app_state
        .lookup_service
        .create::<band::Entity, band::Model, band::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_bands(State(app_state): State<AppState>) -> Result<Response> {
    let novelties = app_state
        .lookup_service
        .find::<band::Entity, band::Model, band::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&novelties));
    Ok((StatusCode::OK, Json(novelties)).into_response())
}

pub async fn create_institution(
    State(app_state): State<AppState>,
    Json(institution): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = institution::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(institution.name),
    };

    app_state
        .lookup_service
        .create::<institution::Entity, institution::Model, institution::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_institutions(State(app_state): State<AppState>) -> Result<Response> {
    let institutions = app_state
        .lookup_service
        .find::<institution::Entity, institution::Model, institution::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&institutions));
    Ok((StatusCode::OK, Json(institutions)).into_response())
}

pub async fn create_profession(
    State(app_state): State<AppState>,
    Json(profession): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = profession::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(profession.name),
    };

    app_state
        .lookup_service
        .create::<profession::Entity, profession::Model, profession::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_professions(State(app_state): State<AppState>) -> Result<Response> {
    let professions = app_state
        .lookup_service
        .find::<profession::Entity, profession::Model, profession::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&professions));
    Ok((StatusCode::OK, Json(professions)).into_response())
}

pub async fn get_divisions(State(app_state): State<AppState>) -> Result<Response> {
    let divisions = app_state.lookup_service.find_divisions().await?;

    debug!("{:?}", Json(&divisions));
    Ok((StatusCode::OK, Json(divisions)).into_response())
}

pub async fn create_base(
    State(app_state): State<AppState>,
    Json(base): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = base::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(base.name),
    };

    app_state
        .lookup_service
        .create::<base::Entity, base::Model, base::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_bases(State(app_state): State<AppState>) -> Result<Response> {
    let bases = app_state
        .lookup_service
        .find::<base::Entity, base::Model, base::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&bases));
    Ok((StatusCode::OK, Json(bases)).into_response())
}
