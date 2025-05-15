use application::dtos::lookup::CreateBasicLookUpDTO;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use domain::{
    entities::{brigade, division, organism},
    repositories::lookup_repository::LookupRepository,
};
use tracing::{debug, error};

use crate::state::AppState;

pub async fn get_brigades(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state
        .lookup_repository
        .find::<brigade::Entity, brigade::Model, brigade::ActiveModel>()
        .await
    {
        Ok(brigades) => {
            debug!("{:?}", Json(&brigades));

            (StatusCode::OK, Json(brigades)).into_response()
        }
        Err(e) => {
            error!("Error getting brigades: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_brigade(
    State(app_state): State<AppState>,
    Json(brigade): Json<CreateBasicLookUpDTO>,
) -> impl IntoResponse {
    let active_model = brigade::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(brigade.name),
    };

    match app_state
        .lookup_repository
        .create::<brigade::Entity, brigade::Model, brigade::ActiveModel>(active_model)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            error!("Error creating brigade: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_organisms(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state
        .lookup_repository
        .find::<organism::Entity, organism::Model, organism::ActiveModel>()
        .await
    {
        Ok(organisms) => {
            debug!("{:?}", Json(&organisms));

            (StatusCode::OK, Json(organisms)).into_response()
        }
        Err(e) => {
            error!("Error getting organisms: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_organism(
    State(app_state): State<AppState>,
    Json(organism): Json<CreateBasicLookUpDTO>,
) -> impl IntoResponse {
    let active_model = organism::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(organism.name),
        phone: sea_orm::Set(String::new()), // Default empty string for phone
    };

    match app_state
        .lookup_repository
        .create::<organism::Entity, organism::Model, organism::ActiveModel>(active_model)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            error!("Error creating organism: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_divisions(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state
        .lookup_repository
        .find::<division::Entity, division::Model, division::ActiveModel>()
        .await
    {
        Ok(divisions) => {
            debug!("{:?}", Json(&divisions));

            (StatusCode::OK, Json(divisions)).into_response()
        }
        Err(e) => {
            error!("Error getting divisions: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_division(
    State(app_state): State<AppState>,
    Json(division): Json<CreateBasicLookUpDTO>,
) -> impl IntoResponse {
    let active_model = division::ActiveModel {
        id: Default::default(),
        name: sea_orm::Set(division.name),
        state: sea_orm::Set(String::new()), // Default empty string for state
    };

    match app_state
        .lookup_repository
        .create::<division::Entity, division::Model, division::ActiveModel>(active_model)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            error!("Error creating division: {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
