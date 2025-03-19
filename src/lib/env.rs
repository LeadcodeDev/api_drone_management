use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub database_url: String,

    #[clap(env)]
    pub host: String,

    #[clap(env)]
    pub port: u16,

    #[clap(env)]
    pub auth_service_url: String,

    #[clap(env)]
    pub keycloak_client_secret: String,
}
