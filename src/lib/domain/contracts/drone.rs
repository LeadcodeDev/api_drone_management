use crate::domain::models::drone::Drone;
use std::future::Future;

pub struct DronePayload {
    pub model: String,
    pub capacity: i32,
}

pub trait DroneRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>> + Send;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn store(
        &self,
        payload: DronePayload,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn update(
        &self,
        id: i32,
        payload: DronePayload,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn delete(&self, id: i32) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait DroneService: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, anyhow::Error>> + Send;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn store(
        &self,
        payload: DronePayload,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn update(
        &self,
        id: i32,
        payload: DronePayload,
    ) -> impl Future<Output = Result<Drone, anyhow::Error>> + Send;

    fn delete(&self, id: i32) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}
