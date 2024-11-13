use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub object: String,
    pub url: String,
    pub data_updated_at: DateTime<Utc>,
    pub data: SummaryData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryData {
    pub lessons: Vec<LessonBlock>,
    pub reviews: Vec<ReviewBlock>,
    pub next_reviews_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonBlock {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewBlock {
    pub available_at: DateTime<Utc>,
    pub subject_ids: Vec<u64>,
}

impl Summary {
    pub fn get_available_lessons(&self) -> Vec<u64> {
        self.data
            .lessons
            .iter()
            .flat_map(|block| block.subject_ids.clone())
            .collect()
    }

    pub fn get_available_reviews(&self) -> Vec<u64> {
        self.data
            .reviews
            .iter()
            .flat_map(|block| block.subject_ids.clone())
            .collect()
    }
}
