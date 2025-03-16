use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::application::errors::ApiError;
use crate::domain::entities::http_error::HttpError;

pub struct ApiResponse<T: Serialize> (StatusCode, Json<BodyResponse<T>>);

impl<T: Serialize> IntoResponse for ApiResponse<T> {
  fn into_response(self) -> Response {
    (self.0, self.1).into_response()
  }
}

impl <T: Serialize> ApiResponse<T> {
  pub fn ok(data: T) -> Self {
    Self(StatusCode::OK, Json(BodyResponse::new(data)))
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct BodyResponse<T: Serialize> {
  status_code: u16,
  data: T,
}

impl <T: Serialize> BodyResponse<T> {
  pub fn new(data: T) -> Self {
    Self {
      status_code: 200,
      data,
    }
  }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiResponseError {
  pub code: String,
  pub status: u16,
  pub message: String,
}

impl From<HttpError> for ApiError {
  fn from(value: HttpError) -> Self {
    match value {
      HttpError::InternalServerError => {
        ApiError::InternalServerError("Internal server error".to_string())
      }
      HttpError::NotFound => ApiError::NotFound("Resource not found".to_string()),
      HttpError::Unauthorized => ApiError::Unauthorized("Unauthorized".to_string()),
      HttpError::Forbidden => ApiError::Forbidden("Forbidden".to_string()),
    }
  }
}