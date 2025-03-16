use crate::domain::models::drone::Drone;
use crate::domain::{contracts::drone::DroneRepository, models::drone::DroneError};
use crate::infrastructure::db::postgres::Postgres;
use sqlx::{Executor, Row};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PostgresDroneRepository {
    db: Arc<Postgres>,
}

impl PostgresDroneRepository {
    pub fn new(db: Arc<Postgres>) -> Self {
        Self { db }
    }
}

impl DroneRepository for PostgresDroneRepository {
    async fn get_all(&self) -> Result<Vec<Drone>, DroneError> {
        let pool = self.db.get_pool();
        let rows = pool
            .fetch_all("SELECT * FROM drones")
            .await
            .map_err(|e| DroneError::InternalServerError(e.to_string()))?;
        let drones = rows
            .iter()
            .map(|row| Drone::new(row.get("id"), row.get("model")))
            .collect();

        Ok(drones)
    }
}
