use crate::application::authentication::middlewares::AuthenticationMiddleware;
use reqwest::Client;
use tower::Layer;

#[derive(Clone)]
pub struct AuthenticationLayer {
    pub client: Client,
    pub auth_service_url: String,
    pub auth_service_secret: String,
}

impl AuthenticationLayer {
    pub fn new(client: Client, auth_service_url: String, auth_service_secret: String) -> Self {
        Self {
            client,
            auth_service_url,
            auth_service_secret,
        }
    }
}

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationMiddleware::new(
            inner,
            self.client.clone(),
            self.auth_service_url.clone(),
            self.auth_service_secret.clone(),
        )
    }
}
