use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drone {
    pub id: i32,
    pub model: String,
}

impl Drone {
    pub fn new(id: i32, model: String) -> Self {
        Self { id, model }
    }
}
