use crate::domain::contracts::drone::DroneService;
use crate::domain::models::drone::Drone;
use axum::Extension;
use std::sync::Arc;

pub async fn index<T: DroneService>(
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<Vec<Drone>, anyhow::Error> {
    drone_service.get_all().await
}

pub async fn show<T: DroneService>(
    Extension(service): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn store<T: DroneService>(
    Extension(service): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn update<T: DroneService>(
    Extension(service): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn destroy<T: DroneService>(
    Extension(service): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}
