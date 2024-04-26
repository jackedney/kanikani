use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Client;
use std::env;

const BASE_URL: &str = "https://api.wanikani.com/v2";

pub struct WaniKaniClient {
    client: Client,
    api_token: String,
}

impl WaniKaniClient {
    pub fn new(api_token: String) -> Self {
        let client = Client::new();
        WaniKaniClient { client, api_token }
    }

    pub async fn authenticate(&self) -> Result<bool, reqwest::Error> {
        let url = format!("{}/user", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response = self.client.get(&url).headers(headers).send().await?;

        Ok(response.status().is_success())
    }

    // Other API interaction functions will be added here
}

pub async fn create_client() -> Result<WaniKaniClient, Box<dyn std::error::Error>> {
    let api_token = env::var("WANIKANI_API_TOKEN")?;
    let client = WaniKaniClient::new(api_token);

    if client.authenticate().await? {
        Ok(client)
    } else {
        Err("Authentication failed".into())
    }
}
