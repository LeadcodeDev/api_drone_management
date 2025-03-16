use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drone {
    pub id: i32,
    pub model: String,
  pub capacity: i32,
}

impl Drone {
    pub fn new(id: i32, model: String, capacity: i32) -> Self {
        Self { id, model, capacity }
    }
}
