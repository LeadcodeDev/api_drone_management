use crate::application::responses::ApiResponse;
use crate::domain::contracts::drone::DroneService;
use crate::domain::models::drone::Drone;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;
use crate::application::errors::ApiError;

#[derive(Debug, Clone, Serialize)]
pub struct FetchDroneResponse(Vec<Drone>);

pub async fn index<T: DroneService>(
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<ApiResponse<FetchDroneResponse>, ApiError> {
    drone_service
        .get_all()
        .await
        .map_err(ApiError::from)
        .map(|drone| ApiResponse::ok(FetchDroneResponse(drone)))
}

pub async fn show<T: DroneService>(
    Extension(_): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn store<T: DroneService>(
    Extension(_): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn update<T: DroneService>(
    Extension(_): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}

pub async fn destroy<T: DroneService>(
    Extension(_): Extension<Arc<T>>,
) -> Result<Drone, anyhow::Error> {
    unimplemented!()
}
