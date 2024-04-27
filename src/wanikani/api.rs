use crate::wanikani::user::User;
use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Client;

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

    pub async fn authenticate(&self) -> Result<()> {
        let url = format!("{}/user", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response = self.client.get(&url).headers(headers).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!(
                "Authentication failed with status code: {}",
                response.status()
            ))
        }
    }
    pub async fn fetch_user_info(&self) -> Result<User> {
        let url = format!("{}/user", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response_body = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let user_info: User = serde_json::from_str(&response_body)?;
        Ok(user_info)
    }

    // Add more methods for fetching reviews, lessons, etc.
}
