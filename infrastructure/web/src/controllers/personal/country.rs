use crate::tags::COUNTRY_TAG;
use crate::Result;
use application::api::ApiContext;
use application::dtos::personal::country::AddVerificationDTO;
use application::services::personal::country::CountryService;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountryVerificationBody<T> {
    pub verification: T,
}

pub async fn add_verification(
    Extension(ctx): Extension<ApiContext>,
    Json(dto): Json<AddVerificationDTO>,
) -> Result<Response> {
    let verification_id = CountryService::add_verification(ctx, dto).await?;

    Ok((
        StatusCode::CREATED,
        Json(CountryVerificationBody::<i32> {
            verification: verification_id,
        }),
    )
        .into_response())
}

#[utoipa::path(get, path = "/", tag = COUNTRY_TAG,
    responses(
    (status = 200, description = "Registro actualizado de forma correcta"),
)
)]
pub async fn get_country_csv(Extension(ctx): Extension<ApiContext>) -> Result<Response> {
    let personas = CountryService::get_country_csv(ctx).await?;

    let mut wtr = csv::Writer::from_writer(vec![]);

    wtr.write_record(["Cedula", "Genero"])?;

    for persona in personas {
        wtr.write_record(&[persona.ci, persona.genre])?;
    }

    wtr.flush()?;

    let file_content = String::from_utf8(wtr.get_ref().to_vec()).unwrap();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/csv; charset=utf-8")
        .header("Content-Disposition", "attachment; filename=\"data.csv\"")
        .body(file_content)
        .unwrap()
        .into_response();

    Ok(response)
}
