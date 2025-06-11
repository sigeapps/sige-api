use crate::state::AppState;
use crate::Result;
use application::dtos::personal::persona::CreatePersonaDTO;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PersonaBody<T> {
    persona: T,
}

pub async fn create_persona(
    State(app_state): State<AppState>,
    Json(persona): Json<CreatePersonaDTO>,
) -> Result<Response> {
    let persona_id = app_state.persona_service.create(persona).await?;

    Ok((
        StatusCode::CREATED,
        Json(PersonaBody::<i32> {
            persona: persona_id,
        }),
    )
        .into_response())
}
