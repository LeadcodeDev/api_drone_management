use crate::application::authentication::handlers::verify_token::verify_token;
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::Response;
use futures_util::future::BoxFuture;
use reqwest::Client;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct AuthenticationMiddleware<S> {
    inner: S,
    client: Client,
    auth_service_url: String,
    auth_service_secret: String,
}

impl<S> AuthenticationMiddleware<S> {
    pub fn new(
        inner: S,
        client: Client,
        auth_service_url: String,
        auth_service_secret: String,
    ) -> Self {
        Self {
            inner,
            client,
            auth_service_url,
            auth_service_secret,
        }
    }
}

impl<S> Service<Request> for AuthenticationMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let client = self.client.clone();
        let auth_service_url = self.auth_service_url.clone();
        let auth_service_secret = self.auth_service_secret.clone();
        let mut inner = self.inner.clone();

        let token = request
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                header
                    .strip_prefix("Bearer ")
                    .map(|token| token.to_string())
            });

        Box::pin(async move {
            match token {
                Some(token) => {
                    match verify_token(client, &auth_service_url, &auth_service_secret, &token)
                        .await
                    {
                        Ok(data) => {
                            request.extensions_mut().insert(data);
                            inner.call(request).await
                        }
                        Err(_) => {
                            let response = Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body(Body::from("Invalid token"))
                                .unwrap();
                            Ok(response)
                        }
                    }
                }
                None => {
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Missing token"))
                        .unwrap();
                    Ok(response)
                }
            }
        })
    }
}
