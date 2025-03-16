use crate::application::drone::router::drone_router;
use crate::domain::contracts::drone::DroneService;
use crate::env::Env;
use anyhow::Context;
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::application::http::app_state::AppState;

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
        let state = AppState::new(drone_service);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", env.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", env.port))
            .expect("TODO: panic message");

        let router = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .merge(drone_router())
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
