use crate::domain::contracts::drone::DroneService;
use crate::domain::models::drone::Drone;
use axum::Extension;
use axum_extra::routing::TypedPath;
use serde::Serialize;
use std::sync::Arc;
use crate::application::http::errors::{HttpError};
use crate::application::http::responses::Response;

#[derive(Serialize, TypedPath)]
#[typed_path("/drones")]
pub struct FetchDronesRoute;

#[derive(Debug, Clone, Serialize)]
pub struct FetchDroneResponse(Vec<Drone>);

pub async fn index<T: DroneService>(
    _: FetchDronesRoute,
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<Response<FetchDroneResponse>, HttpError> {
    drone_service
        .get_all()
        .await
        .map_err(HttpError::from)
        .map(|drones| Response::ok(FetchDroneResponse(drones)))
}
