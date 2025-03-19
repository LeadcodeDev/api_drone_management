use crate::application::authentication::layer::AuthenticationLayer;
use crate::application::drone::router::drone_router;
use crate::application::http::app_state::AppState;
use crate::domain::contracts::drone::DroneService;
use crate::env::Env;
use anyhow::Context;
use axum::routing::get;
use axum::{Extension, Router};
use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct HttpServer {
    env: Arc<Env>,
    listener: TcpListener,
    router: Router,
}

impl HttpServer {
    pub async fn new<D>(env: Arc<Env>, drone_service: Arc<D>) -> anyhow::Result<Self>
    where
        D: DroneService,
    {
        let client = Client::new();
        let state = AppState::new(drone_service);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", env.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", env.port))
            .expect("Failed to bind to port");

        let auth_layer = AuthenticationLayer::new(
            client,
            env.auth_service_url.clone(),
            env.keycloak_client_secret.clone(),
        );

        let router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .merge(drone_router())
            .layer(auth_layer)
            .layer(Extension(Arc::clone(&state.drone_service)))
            .with_state(state);

        Ok(Self {
            router,
            listener,
            env,
        })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        println!(
            "Starting HTTP server on {}:{}",
            self.env.host, self.env.port
        );

        axum::serve(self.listener, self.router)
            .await
            .context("Http server error")?;

        Ok(())
    }
}
