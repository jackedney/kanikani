use crate::wanikani::assignment::AssignmentCollection;
use crate::wanikani::subject::subject;
use crate::wanikani::summary;
use crate::wanikani::user::User;
use anyhow::{anyhow, Result};
use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, AUTHORIZATION};

const BASE_URL: &str = "https://api.wanikani.com/v2";

pub struct WaniKaniClient {
    client: BlockingClient,
    api_token: String,
}

impl Clone for WaniKaniClient {
    fn clone(&self) -> Self {
        WaniKaniClient {
            client: BlockingClient::new(),
            api_token: self.api_token.clone(),
        }
    }
}

impl WaniKaniClient {
    pub fn new(api_token: String) -> Self {
        let client = BlockingClient::new();
        WaniKaniClient { client, api_token }
    }

    pub fn authenticate(&self) -> Result<()> {
        let url = format!("{}/user", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response = self.client.get(&url).headers(headers).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!(
                "Authentication failed with status code: {}",
                response.status()
            ))
        }
    }

    pub fn fetch_user_info(&self) -> Result<User> {
        let url = format!("{}/user", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response_body = self.client.get(&url).headers(headers).send()?.text()?;

        let user_info: User = serde_json::from_str(&response_body)?;
        Ok(user_info)
    }

    pub fn fetch_assignments(&self) -> Result<AssignmentCollection> {
        let url = format!("{}/assignments", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response_body = self.client.get(&url).headers(headers).send()?.text()?;

        let assignments = serde_json::from_str(&response_body);
        match assignments {
            Err(e) => {
                println!("Response body: {:#?}", &response_body);
                return Err(e.into());
            }
            Ok(assignments) => Ok(assignments),
        }
    }

    pub fn fetch_subject(&self, subject_id: u64) -> Result<subject::Subject> {
        let url = format!("{}/subjects/{}", BASE_URL, subject_id);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response_body = self.client.get(&url).headers(headers).send()?.text()?;

        let subject = serde_json::from_str(&response_body);
        match subject {
            Err(e) => {
                println!("Response body: {:#?}", &response_body);
                return Err(e.into());
            }
            Ok(subject) => Ok(subject),
        }
    }
    pub fn fetch_summary(&self) -> Result<summary::Summary> {
        let url = format!("{}/summary", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        let response_body = self.client.get(&url).headers(headers).send()?.text()?;

        let summary = serde_json::from_str(&response_body);
        match summary {
            Err(e) => {
                println!("Response body: {:#?}", &response_body);
                return Err(e.into());
            }
            Ok(summary) => Ok(summary),
        }
    }

    pub fn fetch_available_assignments(
        &self,
        immediately_available: bool,
    ) -> Result<AssignmentCollection> {
        let mut url = format!("{}/assignments", BASE_URL);
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_token).parse().unwrap(),
        );

        if immediately_available {
            url.push_str("?immediately_available_for_review=true");
        }

        let response_body = self.client.get(&url).headers(headers).send()?.text()?;

        let assignments = serde_json::from_str(&response_body);
        match assignments {
            Err(e) => {
                println!("Response body: {:#?}", &response_body);
                return Err(e.into());
            }
            Ok(assignments) => Ok(assignments),
        }
    }

    pub fn submit_review(&self, review_data: serde_json::Value) -> Result<()> {
        let url = format!("{}/reviews", BASE_URL);
        self.client.post(&url).json(&review_data).send()?;
        Ok(())
    }

    // Add more methods for fetching reviews, lessons, etc.
}
