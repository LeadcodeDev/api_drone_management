use crate::application::http::errors::HttpError;
use crate::application::http::responses::Response;
use crate::domain::contracts::drone::{DronePayload, DroneService};
use crate::domain::models::drone::Drone;
use axum::{Extension, Json};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, TypedPath)]
#[typed_path("/drones")]
pub struct StoreDronesRoute;

#[derive(Debug, Clone, Serialize)]
pub struct StoreDroneResponse(Drone);

#[derive(Debug, Clone, Deserialize)]
pub struct StoreDroneRequest {
    pub model: String,
    pub capacity: i32,
}

pub async fn store<T: DroneService>(
    _: StoreDronesRoute,
    Extension(drone_service): Extension<Arc<T>>,
    Json(payload): Json<StoreDroneRequest>,
) -> Result<Response<StoreDroneResponse>, HttpError> {
    drone_service
        .store(DronePayload {
            model: payload.model,
            capacity: payload.capacity,
        })
        .await
        .map_err(HttpError::from)
        .map(|drone| Response::created(StoreDroneResponse(drone)))
}
