use axum::{http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

use crate::domain::models::drone::DroneError;

use super::responses::GenericResponse;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse::new(&message)),
            )
                .into_response(),
        }
    }
}

impl From<DroneError> for ApiError {
    fn from(error: DroneError) -> Self {
        match error {
            DroneError::InternalServerError(message) => Self::InternalServerError(message),
        }
    }
}
