use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    status_code: u16,
    message: String,
}

impl From<anyhow::Error> for HttpError {
    fn from(error: anyhow::Error) -> Self {
        match error {
            error if error.to_string().contains("E_VALIDATION") => Self {
                status_code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                message: error.to_string(),
            },
            _ => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: error.to_string(),
            },
        }
    }
}

impl HttpError {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self {
            status_code: status_code.as_u16(),
            message: message.to_string(),
        }
    }

    pub fn unprocessable(body: &str) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, body)
    }

    pub fn not_found(body: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, body)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.status_code).unwrap(),
            Json(self.message),
        )
            .into_response()
    }
}
