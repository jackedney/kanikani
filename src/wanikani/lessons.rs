use crate::display;
use crate::wanikani::subject::subject;
use crate::wanikani::utils;
use crate::WaniKaniClient;
use anyhow::Result;
use okanimoji::{generate_ascii_image, generate_ascii_text};

pub struct LessonSession {
    client: WaniKaniClient,
    subject_ids: Vec<u64>,
    current_index: usize,
    display_method: String,
}

impl LessonSession {
    pub fn new(client: WaniKaniClient, subject_ids: Vec<u64>, display_method: String) -> Self {
        LessonSession {
            client,
            subject_ids,
            current_index: 0,
            display_method,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.subject_ids.is_empty() {
            display::display_text(&self.display_method, "No lessons available!");
            return Ok(());
        }

        display::display_text(
            &self.display_method,
            &format!(
                "Starting lessons session with {} items",
                self.subject_ids.len()
            ),
        );

        while self.current_index < self.subject_ids.len() {
            self.show_current_lesson()?;
            let command = display::text_input(
                &self.display_method,
                "Press Enter for next lesson, 'q' to quit",
            );
            if command == "q" {
                break;
            }
            self.current_index += 1;
        }

        display::display_text(&self.display_method, "Lesson session complete!");
        Ok(())
    }

    fn show_current_lesson(&self) -> Result<()> {
        let subject_id = self.subject_ids[self.current_index];
        let subject = self.client.fetch_subject(subject_id)?;

        // Display the character
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

        // Display information
        match &subject.data {
            subject::SubjectData::Radical(radical) => {
                display::display_text(
                    &self.display_method,
                    &format!(
                        "\nMeaning: {}\nMnemonic: {}\n",
                        radical
                            .meanings
                            .first()
                            .map(|m| &m.meaning)
                            .unwrap_or(&String::new()),
                        radical.meaning_mnemonic
                    ),
                );
            }
            subject::SubjectData::Kanji(kanji) => {
                display::display_text(
                    &self.display_method,
                    &format!(
                        "\nMeaning: {}\nReading (hiragana or romaji): {}\nMeaning Mnemonic: {}\nReading Mnemonic: {}\n",
                        kanji
                            .meanings
                            .first()
                            .map(|m| &m.meaning)
                            .unwrap_or(&String::new()),
                        kanji
                            .readings
                            .first()
                            .map(|r| &r.reading)
                            .unwrap_or(&String::new()),
                        kanji.meaning_mnemonic,
                        kanji.reading_mnemonic
                    ),
                );
            }
            subject::SubjectData::Vocabulary(vocab) => {
                display::display_text(
                    &self.display_method,
                    &format!(
                        "\nMeaning: {}\nReading (hiragana or romaji): {}\nMeaning Mnemonic: {}\nReading Mnemonic: {}\n",
                        vocab
                            .meanings
                            .first()
                            .map(|m| &m.meaning)
                            .unwrap_or(&String::new()),
                        vocab
                            .readings
                            .first()
                            .map(|r| &r.reading)
                            .unwrap_or(&String::new()),
                        vocab.meaning_mnemonic,
                        vocab.reading_mnemonic
                    ),
                );
            }
            subject::SubjectData::KanaVocabulary(vocab) => {
                display::display_text(
                    &self.display_method,
                    &format!(
                        "\nMeaning: {}\nMeaning Mnemonic: {}\n",
                        vocab
                            .meanings
                            .first()
                            .map(|m| &m.meaning)
                            .unwrap_or(&String::new()),
                        vocab.meaning_mnemonic
                    ),
                );
            }
        }

        Ok(())
    }
}



