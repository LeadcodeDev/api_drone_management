use crate::domain::models::drone::Drone;
use std::future::Future;

pub trait DroneRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>> + Send;
    fn store(
        &self,
        model: String,
        capacity: i32,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;
  
    fn update(
        &self,
        id: i32,
        model: String,
        capacity: i32,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;
}

pub trait DroneService: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>> + Send;
    fn store(
        &self,
        model: String,
        capacity: i32,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;
  
    fn update(
        &self,
        id: i32,
        model: String,
        capacity: i32,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;
}
