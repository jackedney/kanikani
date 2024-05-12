use crate::wanikani::assignment::AssignmentCollection;
use crate::wanikani::subject::Subject;
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

    pub async fn fetch_assignments(&self) -> Result<AssignmentCollection> {
        let url = format!("{}/assignments", BASE_URL);
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

        let assignments: AssignmentCollection = serde_json::from_str(&response_body)?;
        Ok(assignments)
    }

    pub async fn fetch_subject(&self, subject_id: u64) -> Result<Subject> {
        let url = format!("{}/subjects/{}", BASE_URL, subject_id);
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

        let debug_json: serde_json::Value = serde_json::from_str(&response_body)?;
        println!("{}", serde_json::to_string_pretty(&debug_json).unwrap());

        let subject: Subject = serde_json::from_str(&response_body)?;
        Ok(subject)
    }

    // Add more methods for fetching reviews, lessons, etc.
}
