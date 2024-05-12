use crate::wanikani::decode::{from_rfc3339, from_rfc3339_option};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub object: Option<String>,
    pub url: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub data_updated_at: DateTime<Utc>,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub level: u8,
    pub profile_url: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub started_at: DateTime<Utc>,
    pub subscription: Subscription,
    #[serde(deserialize_with = "from_rfc3339_option")]
    pub current_vacation_started_at: Option<DateTime<Utc>>,
    pub preferences: Preferences,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subscription {
    pub active: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub max_level_granted: u8,
    #[serde(deserialize_with = "from_rfc3339")]
    pub period_ends_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Preferences {
    pub lessons_autoplay_audio: bool,
    pub lessons_batch_size: u8,
    pub reviews_autoplay_audio: bool,
    pub reviews_display_srs_indicator: bool,
    pub extra_study_autoplay_audio: bool,
    pub reviews_presentation_order: String,
    pub lessons_presentation_order: String,
    pub default_voice_actor_id: u8,
}
