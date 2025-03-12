use crate::domain::contracts::drone::DroneRepository;
use crate::domain::models::drone::Drone;
use crate::infrastructure::db::postgres::Postgres;
use sqlx::{Executor, Row};
use std::sync::Arc;

pub struct PostgresDroneRepository {
    db: Arc<Postgres>,
}

impl PostgresDroneRepository {
    pub fn new(db: Arc<Postgres>) -> Self {
        Self { db }
    }
}

impl DroneRepository for PostgresDroneRepository {
    async fn get_all(&self) -> Result<Vec<Drone>, anyhow::Error> {
        let pool = self.db.get_pool();
        let rows = pool.fetch_all("SELECT * FROM drones").await?;
        let drones = rows
            .iter()
            .map(|row| Drone::new(row.get("id"), row.get("model")))
            .collect();

        Ok(drones)
    }
}
