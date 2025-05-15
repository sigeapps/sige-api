use crate::Result;
use application::dtos::lookup::CreateBasicLookUpDTO;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::{
    entities::{brigade, charge, division, hierarchy, organism},
    repositories::lookup_repository::LookupRepository,
};
use tracing::{debug, error};

use crate::state::AppState;

pub async fn get_brigades(State(app_state): State<AppState>) -> Result<Response> {
    let brigades = app_state
        .lookup_repository
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
        .lookup_repository
        .create::<brigade::Entity, brigade::Model, brigade::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_organisms(State(app_state): State<AppState>) -> Result<Response> {
    let organisms = app_state
        .lookup_repository
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
        .lookup_repository
        .create::<organism::Entity, organism::Model, organism::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_divisions(State(app_state): State<AppState>) -> Result<Response> {
    let divisions = app_state
        .lookup_repository
        .find::<division::Entity, division::Model, division::ActiveModel>()
        .await?;

    debug!("{:?}", Json(&divisions));
    Ok((StatusCode::OK, Json(divisions)).into_response())
}

pub async fn create_division(
    State(app_state): State<AppState>,
    Json(division): Json<CreateBasicLookUpDTO>,
) -> Result<Response> {
    let active_model = division::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(division.name),
        state: sea_orm::Set(String::new()), // Default empty string for state
    };

    app_state
        .lookup_repository
        .create::<division::Entity, division::Model, division::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_charges(State(app_state): State<AppState>) -> Result<Response> {
    let charges = app_state
        .lookup_repository
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
        .lookup_repository
        .create::<charge::Entity, charge::Model, charge::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_hierarchies(State(app_state): State<AppState>) -> Result<Response> {
    let hierarchies = app_state
        .lookup_repository
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
        .lookup_repository
        .create::<hierarchy::Entity, hierarchy::Model, hierarchy::ActiveModel>(active_model)
        .await?;

    Ok(StatusCode::CREATED.into_response())
}
