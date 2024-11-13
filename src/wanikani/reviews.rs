use crate::display;
use crate::wanikani::subject::subject;
use crate::wanikani::utils;
use crate::WaniKaniClient;
use anyhow::Result;
use okanimoji::{generate_ascii_image, generate_ascii_text};
use std::collections::HashMap;

#[derive(Clone)]
struct ReviewItem {
    subject_id: u64,
    assignment_id: u64,
    incorrect_meaning_answers: u32,
    incorrect_reading_answers: u32,
    needs_meaning: bool,
    needs_reading: bool,
}

pub struct ReviewSession {
    client: WaniKaniClient,
    assignments: HashMap<u64, ReviewItem>,
    current_item: Option<u64>,
    display_method: String,
}

impl ReviewSession {
    pub fn new(
        client: WaniKaniClient,
        assignment_ids: Vec<(u64, u64)>,
        display_method: String,
    ) -> Self {
        let assignments = assignment_ids
            .into_iter()
            .map(|(assignment_id, subject_id)| {
                (
                    subject_id,
                    ReviewItem {
                        subject_id,
                        assignment_id,
                        incorrect_meaning_answers: 0,
                        incorrect_reading_answers: 0,
                        needs_meaning: true,
                        needs_reading: true,
                    },
                )
            })
            .collect();

        ReviewSession {
            client,
            assignments,
            current_item: None,
            display_method,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.assignments.is_empty() {
            display::display_text(&self.display_method, "No reviews available!");
            return Ok(());
        }

        display::display_text(
            &self.display_method,
            &format!(
                "Starting review session with {} items",
                self.assignments.len()
            ),
        );

        while !self.assignments.is_empty() {
            self.select_next_item();
            self.process_current_item()?;
        }

        display::display_text(&self.display_method, "Review session complete!");
        Ok(())
    }

    fn select_next_item(&mut self) {
        self.current_item = self.assignments.keys().next().copied();
    }

    fn process_current_item(&mut self) -> Result<()> {
        let subject_id = match self.current_item {
            Some(id) => id,
            None => return Ok(()),
        };

        let item = self.assignments.get(&subject_id).cloned().unwrap();
        let subject = self.client.fetch_subject(item.subject_id)?;
        let mut item_updated = false;

        if item.needs_meaning {
            let processed_item = self.process_meaning(&subject, item.clone())?;
            if let Some(updated_item) = processed_item {
                self.assignments.insert(subject_id, updated_item);
                item_updated = true;
            }
        }

        let item = self.assignments.get(&subject_id).cloned().unwrap();
        if item.needs_reading {
            let processed_item = self.process_reading(&subject, item)?;
            if let Some(updated_item) = processed_item {
                self.assignments.insert(subject_id, updated_item);
                item_updated = true;
            }
        }

        if !item_updated
            || (!self.assignments.get(&subject_id).unwrap().needs_meaning
                && !self.assignments.get(&subject_id).unwrap().needs_reading)
        {
            let final_item = self.assignments.get(&subject_id).unwrap();
            self.submit_review(final_item)?;
            self.assignments.remove(&subject_id);
        }

        Ok(())
    }

    fn process_meaning(
        &mut self,
        subject: &subject::Subject,
        mut item: ReviewItem,
    ) -> Result<Option<ReviewItem>> {
        match &subject.data {
            subject::SubjectData::KanaVocabulary(vocabulary) => {
                let ascii_string =
                    generate_ascii_text(&vocabulary.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Kanji(kanji) => {
                let ascii_string = generate_ascii_text(&kanji.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Vocabulary(vocab) => {
                let ascii_string = generate_ascii_text(&vocab.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Radical(radical) => {
                if let Some(characters) = &radical.characters {
                    let ascii_string = generate_ascii_text(characters, "togoshi-gothic", 90, 2);
                    display::display_text(&self.display_method, ascii_string.as_str());
                } else {
                    let image_url = &radical.character_images[0].url;
                    let response = reqwest::blocking::get(image_url)?;
                    let svg_data = response.text()?;
                    let image_ = utils::utils::svg_to_dynamic_image(&svg_data)?;
                    let ascii_art = generate_ascii_image(&image_, 80, 24, 2);
                    display::display_text(&self.display_method, &ascii_art);
                }
            }
        }

        display::display_text(&self.display_method, "\nEnter the meaning:");
        let answer = display::text_input(&self.display_method, "");
        let correct = self.check_meaning(subject, &answer);

        if correct {
            display::display_text(&self.display_method, "Correct!");
            item.needs_meaning = false;
            Ok(Some(item))
        } else {
            display::display_text(&self.display_method, "Incorrect. Try again.");
            item.incorrect_meaning_answers += 1;
            Ok(Some(item))
        }
    }

    fn process_reading(
        &mut self,
        subject: &subject::Subject,
        mut item: ReviewItem,
    ) -> Result<Option<ReviewItem>> {
        if let subject::SubjectData::Radical(_) = subject.data {
            item.needs_reading = false;
            return Ok(Some(item));
        }

        match &subject.data {
            subject::SubjectData::KanaVocabulary(vocabulary) => {
                let ascii_string =
                    generate_ascii_text(&vocabulary.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Kanji(kanji) => {
                let ascii_string = generate_ascii_text(&kanji.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Vocabulary(vocab) => {
                let ascii_string = generate_ascii_text(&vocab.characters, "togoshi-gothic", 90, 2);
                display::display_text(&self.display_method, ascii_string.as_str());
            }
            subject::SubjectData::Radical(_) => {}
        }

        display::display_text(
            &self.display_method,
            "\nEnter the reading (in hiragana or romaji):",
        );
        let answer = display::text_input(&self.display_method, "");
        let correct = self.check_reading(subject, &answer);

        if correct {
            display::display_text(&self.display_method, "Correct!");
            item.needs_reading = false;
            Ok(Some(item))
        } else {
            display::display_text(&self.display_method, "Incorrect. Try again.");
            item.incorrect_reading_answers += 1;
            Ok(Some(item))
        }
    }

    fn check_meaning(&self, subject: &subject::Subject, answer: &str) -> bool {
        let correct_meanings: Vec<&str> = match &subject.data {
            subject::SubjectData::KanaVocabulary(vocab) => {
                vocab.meanings.iter().map(|m| m.meaning.as_str()).collect()
            }
            subject::SubjectData::Radical(radical) => radical
                .meanings
                .iter()
                .map(|m| m.meaning.as_str())
                .collect(),
            subject::SubjectData::Kanji(kanji) => {
                kanji.meanings.iter().map(|m| m.meaning.as_str()).collect()
            }
            subject::SubjectData::Vocabulary(vocab) => {
                vocab.meanings.iter().map(|m| m.meaning.as_str()).collect()
            }
        };

        let normalized_answer = answer.trim().to_lowercase();
        correct_meanings
            .iter()
            .any(|&m| m.to_lowercase() == normalized_answer)
    }

    fn check_reading(&self, subject: &subject::Subject, answer: &str) -> bool {
        let correct_readings: Vec<String> = match &subject.data {
            subject::SubjectData::KanaVocabulary(vocab) => {
                vec![vocab.characters.clone()]
            }
            subject::SubjectData::Kanji(kanji) => {
                kanji.readings.iter().map(|r| r.reading.clone()).collect()
            }
            subject::SubjectData::Vocabulary(vocab) => {
                vocab.readings.iter().map(|r| r.reading.clone()).collect()
            }
            subject::SubjectData::Radical(_) => {
                vec![]
            }
        };

        utils::utils::validate_reading(answer, &correct_readings)
    }

    fn submit_review(&self, item: &ReviewItem) -> Result<()> {
        let review_data = serde_json::json!({
            "review": {
                "assignment_id": item.assignment_id,
                "incorrect_meaning_answers": item.incorrect_meaning_answers,
                "incorrect_reading_answers": item.incorrect_reading_answers
            }
        });
        self.client.submit_review(review_data)?;
        Ok(())
    }
}

