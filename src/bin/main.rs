use clap::Parser;
use drone::application::http::http_server::HttpServer;
use drone::domain::services::drone_service::DroneServiceImpl;
use drone::env::Env;
use drone::infrastructure::db::postgres::Postgres;
use drone::infrastructure::repositories::drone_repository::PostgresDroneRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let env = Arc::new(Env::parse());

    let database = match Postgres::new(Arc::clone(&env)).await {
        Ok(database) => Arc::new(database),
        Err(err) => {
            panic!("Failed to connect to database: {:?}", err);
        }
    };

    let drone_repository = PostgresDroneRepository::new(Arc::clone(&database));
    let drone_service = Arc::new(DroneServiceImpl::new(drone_repository));

    // Init webserver
    let server = HttpServer::new(Arc::clone(&env), Arc::clone(&drone_service))
        .await
        .unwrap();

    server.start().await.expect("Failed to start server");
}
