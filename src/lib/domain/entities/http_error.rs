use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum HttpError {
  #[error("Resource not found")]
  NotFound,
  
  #[error("Unauthorized resource")]
  Unauthorized,
  
  #[error("Forbidden resource")]
  Forbidden,
  
  #[error("Internal server error")]
  InternalServerError,
}
