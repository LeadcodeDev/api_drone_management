use crate::domain::contracts::drone::DroneService;
use axum::Router;
use axum_extra::routing::RouterExt;
use crate::application::drone::handlers::delete_drones::delete;
use crate::application::drone::handlers::fetch_drones::index;
use crate::application::drone::handlers::store_drones::store;
use crate::application::drone::handlers::update_drones::update;
use crate::application::http::app_state::AppState;

pub fn drone_router<T>() -> Router<AppState<T>>
where
    T: DroneService
{
  Router::new()
    .typed_get(index::<T>)
    .typed_post(store::<T>)
    .typed_put(update::<T>)
    .typed_delete(delete::<T>)
}
