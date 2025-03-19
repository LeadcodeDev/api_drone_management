use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UserPayload {
    pub email: String,
    pub sub: String,
}

pub async fn verify_token(
  client: Client,
    auth_service_url: &str,
    auth_service_secret: &str,
    token: &str,
) -> Result<UserPayload, ()> {
    let response = client
        .post(format!("{}/token/introspect", auth_service_url))
        .form(&[
            ("client_id", "drones_api"),
            ("client_secret", auth_service_secret),
            ("token", token),
        ])
        .send()
        .await
        .map_err(|_| ())?;

    if response.status().is_success() {
        Ok(response.json().await.map_err(|_| ())?)
    } else {
        Err(())
    }
}
