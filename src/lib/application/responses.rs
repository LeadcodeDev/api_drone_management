use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub message: String,
}

impl GenericResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self::new("Internal server error")
    }
}

pub struct OkResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for OkResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self.0)).into_response()
    }
}

#[derive(Serialize)]
pub struct NotFoundResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for NotFoundResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, Json(self.0)).into_response()
    }
}

pub struct CreatedResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for CreatedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self.0)).into_response()
    }
}

pub struct NoContentResponse;

impl IntoResponse for NoContentResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NO_CONTENT, Json(()).into_response()).into_response()
    }
}
