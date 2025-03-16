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
    T: DroneRepository + Copy + Send + Sync + 'static,
{
    async fn get_all(&self) -> Result<Vec<Drone>, anyhow::Error> {
        self.drone_repository.get_all().await
    }
}
