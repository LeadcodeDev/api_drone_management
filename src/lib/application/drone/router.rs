use crate::application::drone::handlers::delete_drone::delete;
use crate::application::drone::handlers::fetch_drones::index;
use crate::application::drone::handlers::get_drone::get;
use crate::application::drone::handlers::store_drone::store;
use crate::application::drone::handlers::update_drone::update;
use crate::application::http::app_state::AppState;
use crate::domain::contracts::drone::DroneService;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn drone_router<T>() -> Router<AppState<T>>
where
    T: DroneService,
{
    Router::new()
        .typed_get(index::<T>)
        .typed_get(get::<T>)
        .typed_post(store::<T>)
        .typed_put(update::<T>)
        .typed_delete(delete::<T>)
}
