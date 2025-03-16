use crate::application::http::errors::HttpError;
use crate::application::http::responses::Response;
use crate::domain::contracts::drone::{DronePayload, DroneService};
use crate::domain::models::drone::Drone;
use axum::{Extension, Json};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, TypedPath)]
#[typed_path("/drones/{id}")]
pub struct UpdateDronesRoute(i32);

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDroneResponse(Drone);

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDroneRequest {
    pub model: String,
    pub capacity: i32,
}

pub async fn update<T: DroneService>(
    UpdateDronesRoute(id): UpdateDronesRoute,
    Extension(drone_service): Extension<Arc<T>>,
    Json(payload): Json<UpdateDroneRequest>,
) -> Result<Response<UpdateDroneResponse>, HttpError> {
    drone_service
        .update(
            id,
            DronePayload {
                model: payload.model,
                capacity: payload.capacity,
            },
        )
        .await
        .map_err(HttpError::from)
        .map(|drone| Response::ok(UpdateDroneResponse(drone)))
}
