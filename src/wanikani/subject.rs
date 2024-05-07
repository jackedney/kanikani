use crate::wanikani::decode::from_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub id: u64,
    pub object: String,
    pub url: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub data_updated_at: DateTime<Utc>,
    pub data: SubjectData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "object", content = "data")]
pub enum SubjectData {
    #[serde(rename = "radical")]
    Radical(RadicalData),
    #[serde(rename = "kanji")]
    Kanji(KanjiData),
    #[serde(rename = "vocabulary")]
    Vocabulary(VocabularyData),
    #[serde(rename = "kana_vocabulary")]
    KanaVocabulary(KanaVocabularyData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RadicalData {
    pub level: u8,
    pub slug: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub created_at: DateTime<Utc>,
    pub characters: Option<String>,
    pub character_images: Vec<CharacterImage>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KanjiData {
    pub level: u8,
    pub slug: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub created_at: DateTime<Utc>,
    pub characters: String,
    pub meanings: Vec<Meaning>,
    pub readings: Vec<Reading>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VocabularyData {
    pub level: u8,
    pub slug: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub created_at: DateTime<Utc>,
    pub characters: String,
    pub meanings: Vec<Meaning>,
    pub readings: Vec<Reading>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KanaVocabularyData {
    pub level: u8,
    pub slug: String,
    #[serde(deserialize_with = "from_rfc3339")]
    pub created_at: DateTime<Utc>,
    pub characters: String,
    pub meanings: Vec<Meaning>,
    pub parts_of_speech: Vec<String>,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterImage {
    pub url: String,
    pub content_type: String,
    pub metadata: CharacterImageMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterImageMetadata {
    pub inline_styles: Option<bool>,
    pub color: Option<String>,
    pub dimensions: Option<String>,
    pub style_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meaning {
    pub meaning: String,

    pub primary: bool,
    pub accepted_answer: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    pub reading: String,
    pub primary: bool,
    pub accepted_answer: bool,

    #[serde(rename = "type")]
    pub type_: Option<String>,
}
