use crate::application::responses::ApiResponseError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnProcessableEntity(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        match e {
            e if e.to_string().contains("validation error") => {
                Self::UnProcessableEntity(e.to_string())
            }
            _ => Self::InternalServerError(e.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponseBody::new_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal Server Error: {}", e),
                )),
            )
                .into_response(),
            ApiError::UnProcessableEntity(errors) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(errors)).into_response()
            }
            ApiError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponseError {
                    code: "E_NOT_FOUND".to_string(),
                    status: 404,
                    message,
                }),
            )
                .into_response(),
            ApiError::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponseError {
                    code: "E_UNAUTHORIZED".to_string(),
                    status: 403,
                    message,
                }),
            )
                .into_response(),
            ApiError::Forbidden(message) => (
                StatusCode::FORBIDDEN,
                Json(ApiResponseError {
                    code: "E_FORBIDDEN".to_string(),
                    status: 403,
                    message,
                }),
            )
                .into_response(),
        }
    }
}
