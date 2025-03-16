use crate::application::http::errors::HttpError;
use crate::application::http::responses::Response;
use crate::domain::contracts::drone::DroneService;
use axum::Extension;
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, TypedPath)]
#[typed_path("/drones/{id}")]
pub struct DeleteDronesRoute(i32);

pub async fn delete<T: DroneService>(
  DeleteDronesRoute(id): DeleteDronesRoute,
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<Response<()>, HttpError> {
    drone_service
        .delete(id)
        .await
        .map_err(HttpError::from)
        .map(|_| Response::ok(()))
}
