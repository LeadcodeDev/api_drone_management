use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    status_code: u16,
    message: T,
}

impl<T: Serialize> Response<T> {
    pub fn new(status_code: StatusCode, message: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            message,
        }
    }

    pub fn ok(body: T) -> Self {
        Self::new(StatusCode::OK, body)
    }

    pub fn not_found(body: T) -> Self {
        Self::new(StatusCode::NOT_FOUND, body)
    }

    pub fn internal_server_error(body: T) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, body)
    }

    pub fn unauthorized(body: T) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, body)
    }

    pub fn forbidden(body: T) -> Self {
        Self::new(StatusCode::FORBIDDEN, body)
    }

    pub fn bad_request(body: T) -> Self {
        Self::new(StatusCode::BAD_REQUEST, body)
    }

    pub fn created(body: T) -> Self {
        Self::new(StatusCode::CREATED, body)
    }

    pub fn no_content(body: T) -> Self {
        Self::new(StatusCode::NO_CONTENT, body)
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.status_code).unwrap(),
            Json(self.message),
        )
            .into_response()
    }
}
