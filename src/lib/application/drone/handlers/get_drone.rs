use crate::application::http::errors::HttpError;
use crate::application::http::responses::Response;
use crate::domain::contracts::drone::DroneService;
use crate::domain::models::drone::Drone;
use axum::Extension;
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, TypedPath)]
#[typed_path("/drones/{id}")]
pub struct GetDroneRoute(i32);

#[derive(Debug, Clone, Serialize)]
pub struct GetDroneResponse(Drone);

pub async fn get<T: DroneService>(
    GetDroneRoute(id): GetDroneRoute,
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<Response<GetDroneResponse>, HttpError> {
    drone_service
        .get_by_id(id)
        .await
        .map_err(HttpError::from)
        .map(|drone| Response::ok(GetDroneResponse(drone)))
}
