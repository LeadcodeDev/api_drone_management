use crate::application::app::App;
use crate::application::drone::drone_controller::{destroy, index, show, store, update};
use crate::domain::contracts::drone::DroneService;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn drone_router<T>() -> Router<App<T>>
where
    T: DroneService + Clone + Send + Sync,
{
    Router::new()
        .route("/drones", get(index::<T>))
        .route("/drones/:id", get(show::<T>))
        .route("/drones", post(store::<T>))
        .route("/drones/:id", put(update::<T>))
        .route("/drones/:id", delete(destroy::<T>))
}
