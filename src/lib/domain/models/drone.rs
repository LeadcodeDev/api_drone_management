use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
pub struct Drone {
    pub id: i32,
    pub model: String,
}

#[derive(Debug, Clone, Error)]
pub enum DroneError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl Drone {
    pub fn new(id: i32, model: String) -> Self {
        Self { id, model }
    }
}
