use gloo_net::http::Request;
use serde::de::DeserializeOwned;

const API_PATH: &str = "/api/v1";
pub struct ApiClient;

impl ApiClient {
    pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
        let url = format!("{API_PATH}{path}");

        let resp: T = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(resp)
    }
}
