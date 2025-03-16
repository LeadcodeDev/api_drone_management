use crate::domain::models::drone::{Drone, DroneError};
use std::future::Future;

pub trait DroneRepository: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, DroneError>> + Send;
}

pub trait DroneService: Clone + Send + Sync + 'static {
    fn get_all(&self) -> impl Future<Output = Result<Vec<Drone>, DroneError>> + Send;
}
