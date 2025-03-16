use crate::domain::models::drone::Drone;
use std::future::Future;

pub trait DroneRepository {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>>;
}

pub trait DroneService: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>>;
}
