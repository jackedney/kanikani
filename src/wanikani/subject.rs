pub mod subject {
    use crate::wanikani::decode::{from_rfc3339, from_rfc3339_option};
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Subject {
        pub id: u64,
        pub url: String,
        #[serde(deserialize_with = "from_rfc3339")]
        pub data_updated_at: DateTime<Utc>,
        #[serde(flatten)]
        pub data: SubjectData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "object", rename_all = "lowercase", content = "data")]
    pub enum SubjectData {
        Radical(RadicalData),
        Kanji(KanjiData),
        Vocabulary(VocabularyData),
        KanaVocabulary(KanaVocabularyData),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RadicalData {
        pub amalgamation_subject_ids: Vec<u64>,
        pub auxiliary_meanings: Vec<Meaning>,
        pub characters: Option<String>,
        pub character_images: Vec<CharacterImage>,
        #[serde(deserialize_with = "from_rfc3339")]
        pub created_at: DateTime<Utc>,
        pub document_url: String,
        #[serde(deserialize_with = "from_rfc3339_option")]
        pub hidden_at: Option<DateTime<Utc>>,
        pub lesson_position: u8,
        pub level: u8,
        pub meanings: Vec<Meaning>,
        pub meaning_mnemonic: String,
        pub slug: String,
        pub spaced_repetition_system_id: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct KanjiData {
        pub amalgamation_subject_ids: Vec<u64>,
        pub auxiliary_meanings: Vec<Meaning>,
        pub characters: String,
        pub component_subject_ids: Vec<u64>,
        #[serde(deserialize_with = "from_rfc3339")]
        pub created_at: DateTime<Utc>,
        pub document_url: String,
        #[serde(deserialize_with = "from_rfc3339_option")]
        pub hidden_at: Option<DateTime<Utc>>,
        pub lesson_position: u8,
        pub level: u8,
        pub meanings: Vec<Meaning>,
        pub meaning_hint: Option<String>,
        pub meaning_mnemonic: String,
        pub readings: Vec<Reading>,
        pub reading_mnemonic: String,
        pub reading_hint: Option<String>,
        pub slug: String,
        pub visually_similar_subject_ids: Vec<u64>,
        pub spaced_repetition_system_id: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct VocabularyData {
        pub auxiliary_meanings: Vec<Meaning>,
        pub characters: String,
        pub component_subject_ids: Vec<u64>,
        pub context_sentences: Vec<ContextSentence>,
        #[serde(deserialize_with = "from_rfc3339")]
        pub created_at: DateTime<Utc>,
        pub document_url: String,
        #[serde(deserialize_with = "from_rfc3339_option")]
        pub hidden_at: Option<DateTime<Utc>>,
        pub lesson_position: u8,
        pub level: u8,
        pub meanings: Vec<Meaning>,
        pub meaning_mnemonic: String,
        pub parts_of_speech: Vec<String>,
        pub pronunciation_audios: Vec<PronunciationAudio>,
        pub readings: Vec<Reading>,
        pub reading_mnemonic: String,
        pub slug: String,
        pub spaced_repetition_system_id: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct KanaVocabularyData {
        #[serde(deserialize_with = "from_rfc3339")]
        pub created_at: DateTime<Utc>,
        pub level: u8,
        pub slug: String,
        #[serde(deserialize_with = "from_rfc3339_option")]
        pub hidden_at: Option<DateTime<Utc>>,
        pub document_url: String,
        pub characters: String,
        pub meanings: Vec<Meaning>,
        pub auxiliary_meanings: Vec<Meaning>,
        pub parts_of_speech: Vec<String>,
        pub meaning_mnemonic: String,
        pub context_sentences: Vec<ContextSentence>,
        pub pronunciation_audios: Vec<PronunciationAudio>,
        pub lesson_position: u8,
        pub spaced_repetition_system_id: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ContextSentence {
        pub en: String,
        pub ja: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PronunciationAudio {
        pub url: String,
        pub metadata: PronunciationAudioMetadata,
        pub content_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PronunciationAudioMetadata {
        gender: String,
        source_id: u64,
        pronunciation: String,
        voice_actor_id: u64,
        voice_actor_name: String,
        voice_description: String,
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
        pub primary: Option<bool>,
        pub accepted_answer: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Reading {
        pub reading: String,
        pub primary: bool,
        pub accepted_answer: bool,

        #[serde(rename = "type")]
        pub type_: Option<String>,
    }
}
