use crate::domain::contracts::drone::DroneService;
use axum::Router;
use axum::routing::post;
use axum_extra::routing::RouterExt;
use crate::application::drone::handlers::fetch_drones::index;
use crate::application::drone::handlers::store_drones::store;
use crate::application::http::app_state::AppState;

pub fn drone_router<T>() -> Router<AppState<T>>
where
    T: DroneService
{
  Router::new()
    .typed_get(index::<T>)
    .typed_post(store::<T>)
    // .route("/drones/:id", get(show::<T>))
    // .route("/drones", post(store::<T>))
    // .route("/drones/:id", put(update::<T>))
    // .route("/drones/:id", delete(destroy::<T>))
}
