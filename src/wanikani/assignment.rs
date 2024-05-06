use crate::wanikani::decode::{from_rfc3339, from_rfc3339_option};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignmentCollection {
    pub object: String,
    pub url: String,
    pub pages: Pages,
    pub total_count: u64,
    #[serde(deserialize_with = "from_rfc3339")]
    pub data_updated_at: DateTime<Utc>,
    pub data: Vec<Assignment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pages {
    pub per_page: u64,
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    pub id: u64,
    pub object: String,
    pub url: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub data_updated_at: DateTime<Utc>,
    pub data: AssignmentData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignmentData {
    #[serde(deserialize_with = "from_rfc3339")]
    pub created_at: DateTime<Utc>,
    pub subject_id: u64,
    pub subject_type: String,
    pub srs_stage: u8,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub unlocked_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub passed_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub burned_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub available_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub resurrected_at: Option<DateTime<Utc>>,
}
