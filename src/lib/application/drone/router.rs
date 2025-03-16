use crate::application::app::AppState;
use crate::domain::contracts::drone::DroneService;
use axum::Router;
use axum_extra::routing::RouterExt;

use super::drone_controller::get_drones;

pub fn drone_router<T>() -> Router<AppState<T>>
where
    T: DroneService,
{
    Router::new().typed_get(get_drones::<T>)
}
