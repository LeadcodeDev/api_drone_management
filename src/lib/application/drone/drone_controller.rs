use crate::application::errors::ApiError;
use crate::application::responses::OkResponse;
use crate::domain::contracts::drone::DroneService;
use crate::domain::models::drone::Drone;
use axum::Extension;
use axum_macros::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct ListDronesResponseData {
    pub drones: Vec<Drone>,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/drones")]
pub struct GetDronesRoute;


pub async fn get_drones<T: DroneService>(
    _: GetDronesRoute,
    Extension(drone_service): Extension<Arc<T>>,
) -> Result<OkResponse<ListDronesResponseData>, ApiError> {
    drone_service
        .get_all()
        .await
        .map_err(ApiError::from)
        .map(|drones| OkResponse(ListDronesResponseData { drones }))
}
