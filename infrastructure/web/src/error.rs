use axum::{extract::rejection, http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum WebError {
    #[error("authentication required")]
    Unauthorized,
    #[error("user doesn't have permissions")]
    Forbidden,
    #[error("request path not found")]
    NotFound,
    #[error("error on database")]
    Database(#[from] DbErr),
    #[error("unprocessable entity")]
    Validation(#[from] rejection::JsonDataError),
    #[error("token management failed")]
    Jwt(#[from] jwt::Error),
}

impl WebError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Self::Unauthorized => self.status_code().into_response(),
            Self::Database(e) => {
                error!("error on database {}", e);

                self.status_code().into_response()
            }
            Self::Forbidden => self.status_code().into_response(),
            Self::NotFound => self.status_code().into_response(),
            Self::Validation(e) => {
                error!("validation error: {}", e);

                self.status_code().into_response()
            }
            Self::Jwt(e) => {
                error!("jwt error: {}", e);

                self.status_code().into_response()
            }
        }
    }
}
