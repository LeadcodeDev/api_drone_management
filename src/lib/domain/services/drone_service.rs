use crate::domain::contracts::drone::{DroneRepository, DroneService};
use crate::domain::models::drone::Drone;

#[derive(Debug, Clone)]
pub struct DroneServiceImpl<T>
where
    T: DroneRepository,
{
    drone_repository: T,
}

impl<T> DroneServiceImpl<T>
where
    T: DroneRepository,
{
    pub fn new(drone_repository: T) -> Self {
        Self { drone_repository }
    }
}

impl<T> DroneService for DroneServiceImpl<T>
where
    T: DroneRepository,
{
    async fn get_all(&self) -> Result<Vec<Drone>, anyhow::Error> {
        self.drone_repository.get_all().await
    }

    async fn store(&self, model: String, capacity: i32) -> Result<Drone, anyhow::Error> {
        self.drone_repository.store(model, capacity).await
    }

    async fn update(&self, id: i32, model: String, capacity: i32) -> Result<Drone, anyhow::Error> {
        self.drone_repository.update(id, model, capacity).await
    }
  
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        self.drone_repository.delete(id).await
    }
}
