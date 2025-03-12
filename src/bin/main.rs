use clap::Parser;
use drone::application::http_server::HttpServer;
use drone::domain::contracts::drone::DroneService;
use drone::domain::services::drone_service::DroneServiceImpl;
use drone::env::Env;
use drone::infrastructure::db::postgres::Postgres;
use drone::infrastructure::repositories::drone_repository::PostgresDroneRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Validate environment
    dotenv::dotenv().ok();
    let env = Arc::new(Env::parse());

    // Init services (repositories, services, etc)
    let database = Arc::new(Postgres::new(Arc::clone(&env)).await.unwrap());
    let drone_repository = Arc::new(PostgresDroneRepository::new(Arc::clone(&database)));
    let drone_service = Arc::new(DroneServiceImpl::new(Arc::clone(&drone_repository)));

    let drones = drone_service.get_all().await.unwrap();
    println!("{:?}", drones);

    // Init webserver
    let server = HttpServer::new(Arc::clone(&env), Arc::clone(&drone_service));
    server.start();
}
