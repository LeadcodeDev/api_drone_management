use crate::domain::contracts::drone::{DronePayload, DroneRepository};
use crate::domain::models::drone::Drone;
use crate::infrastructure::db::postgres::Postgres;
use sqlx::Row;
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
            .map(|row| Drone::new(row.get("id"), row.get("model"), row.get("capacity")))
            .collect();

        Ok(drones)
    }

    async fn get_by_id(&self, id: i32) -> Result<Drone, anyhow::Error> {
        let pool = self.db.get_pool();
        let result = sqlx::query("SELECT * FROM drones WHERE id = $1")
            .bind(id)
            .fetch_one(&*pool)
            .await?;

        let drone = Drone::new(
            result.get("id"),
            result.get("model"),
            result.get("capacity"),
        );

        Ok(drone)
    }

    async fn store(&self, payload: DronePayload) -> Result<Drone, anyhow::Error> {
        let pool = self.db.get_pool();
        let result =
            sqlx::query("INSERT INTO drones (model, capacity) VALUES ($1, $2) RETURNING *")
                .bind(payload.model)
                .bind(payload.capacity)
                .fetch_one(&*pool)
                .await?;

        let drone = Drone::new(
            result.get("id"),
            result.get("model"),
            result.get("capacity"),
        );

        Ok(drone)
    }

    async fn update(&self, id: i32, payload: DronePayload) -> Result<Drone, anyhow::Error> {
        let pool = self.db.get_pool();
        let result =
            sqlx::query("UPDATE drones SET model = $1, capacity = $2 WHERE id = $3 RETURNING *")
                .bind(payload.model)
                .bind(payload.capacity)
                .bind(id)
                .fetch_one(&*pool)
                .await?;

        let drone = Drone::new(
            result.get("id"),
            result.get("model"),
            result.get("capacity"),
        );

        Ok(drone)
    }

    async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        let pool = self.db.get_pool();
        sqlx::query("DELETE FROM drones WHERE id = $1")
            .bind(id)
            .execute(&*pool)
            .await?;

        Ok(())
    }
}
