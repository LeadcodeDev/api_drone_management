use crate::domain::contracts::drone::DroneService;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState<D>
where
    D: DroneService,
{
    pub drone_service: Arc<D>,
}

impl<D> AppState<D>
where
    D: DroneService,
{
    pub fn new(drone_service: Arc<D>) -> Self {
        Self { drone_service }
    }
}
