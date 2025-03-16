use crate::domain::contracts::drone::DroneRepository;
use crate::domain::models::drone::Drone;
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
    async fn get_all(&self) -> Result<Vec<Drone>, anyhow::Error> {
        let pool = self.db.get_pool();
        let rows = sqlx::query("SELECT * FROM drones")
            .fetch_all(&*pool)
            .await?;
      
        let drones = rows
            .iter()
            .map(|row| Drone::new(row.get("id"), row.get("model")))
            .collect();

        Ok(drones)
    }

    async fn store(&self, model: String, capacity: i32) -> Result<Drone, anyhow::Error> {
        let pool = self.db.get_pool();
        let result = sqlx::query(
            "INSERT INTO drones (model, capacity) VALUES ($1, $2) RETURNING *",
        )
        .bind(&model)
        .bind(capacity)
        .fetch_one(&*pool)
        .await?;

        let drone = Drone::new(result.get("id"), result.get("model"));

        Ok(drone)
    }
}
