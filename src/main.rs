mod config;
mod term;
mod tui;
mod wanikani;

use crate::config::{load_config, save_config, Config};
use crate::wanikani::api::WaniKaniClient;
use crate::wanikani::subject::SubjectData;

use okanimoji::generate_ascii_text;

const KANILOGO_PATH: &str = "src/art/kanilogo.txt";
const KANINAME_PATH: &str = "src/art/kaniname.txt";

mod menu {
    use crate::WaniKaniClient;
    pub type MenuAction = fn(&str, &WaniKaniClient) -> ();

    fn placeholder_action(output_method: &str, _client: &WaniKaniClient) {
        println!("{}", output_method);
    }

    pub const INTRO_MENU: &[(&char, &str, MenuAction)] = &[
        (&'0', "Reviews", placeholder_action),
        (&'1', "Lessons", placeholder_action),
        (&'2', "Stats", placeholder_action),
        (&'3', "Dictionary", placeholder_action),
        (&'4', "Settings", placeholder_action),
        (&'5', "Logout", placeholder_action),
        (&'q', "Quit", placeholder_action),
    ];
}

mod display {
    use crate::menu;
    use crate::term;
    use crate::tui;
    use std::cmp;

    use terminal_size::terminal_size;

    fn center_ascii_art(ascii: &str) -> String {
        let mut result = String::new();

        let term_width = match terminal_size() {
            Some((w, _)) => w.0 as usize,
            None => 80, // Provide a default width if terminal size is not available
        };

        let max_line_width = ascii
            .lines()
            .map(|line| line.trim().len())
            .max()
            .unwrap_or(0);

        let left_padding = (term_width - cmp::min(term_width, max_line_width)) / 2;

        for line in ascii.lines() {
            result.push_str(&" ".repeat(left_padding));
            result.push_str(line.trim());
            result.push('\n');
        }
        result
    }

    fn create_start_screen_ascii() -> String {
        let mut logo_ascii = std::fs::read_to_string(super::KANILOGO_PATH).unwrap();
        let mut name_ascii = std::fs::read_to_string(super::KANINAME_PATH).unwrap();

        logo_ascii = center_ascii_art(&logo_ascii);
        name_ascii = center_ascii_art(&name_ascii);

        format!("{}\n{}", logo_ascii, name_ascii)
    }

    pub fn display_start_screen(output_method: &str) {
        let ascii_intro = create_start_screen_ascii();
        match output_method {
            "term" => term::display_start_screen(&ascii_intro),
            "tui" => tui::display_start_screen(&ascii_intro),
            _ => panic!("Invalid output method"),
        }
    }

    pub fn display_menu(output_method: &str, choices: &[(&char, &str, menu::MenuAction)]) -> char {
        let choices_: &[(&char, &str)] = &choices[..]
            .iter()
            .map(|(key, option, _)| (*key, *option))
            .collect::<Vec<_>>();

        match output_method {
            "term" => term::display_menu(choices_),
            "tui" => tui::display_menu(choices_),
            _ => panic!("Invalid output method"),
        }
    }

    pub fn text_input(output_method: &str, msg: &str) -> String {
        match output_method {
            "term" => term::text_input(msg),
            "tui" => tui::text_input(msg),
            _ => panic!("Invalid output method"),
        }
    }

    pub fn display_text(output_method: &str, text: &str) {
        match output_method {
            "term" => term::display_text(text),
            "tui" => tui::display_text(text),
            _ => panic!("Invalid output method"),
        }
    }
}

#[tokio::main]
async fn main() {
    let output_method = "term";
    display::display_start_screen(output_method);

    // Load the configuration or prompt for the API token
    let api_token = match load_config() {
        Some(config) => config.api_token,
        None => {
            let input_msg = "Please enter your WaniKani API token:";
            let api_token = display::text_input(output_method, input_msg);
            save_config(&Config {
                api_token: api_token.clone(),
            });
            api_token
        }
    };

    // Create the WaniKani client
    let client = WaniKaniClient::new(api_token);

    // Authenticate the user
    if let Err(e) = client.authenticate().await {
        display::display_text(
            output_method,
            &String::from(format!("Authentication failed: {}", e)),
        );
        return;
    } else {
        display::display_text(output_method, "Authentication successful!");
    }

    if let Err(e) = client.fetch_assignments().await {
        display::display_text(
            output_method,
            &String::from(format!("Assignment fetching failed: {}", e)),
        );
        return;
    } else if let Ok(assignments) = client.fetch_assignments().await {
        display::display_text(output_method, "Fetch Assignments successful!");
        let assignment = &assignments.data[0];
        let subject = client
            .fetch_subject(assignment.data.subject_id)
            .await
            .unwrap();

        let mut characters_: Option<String> = None;

        match subject.data {
            SubjectData::Radical(ref data) => {
                characters_ = Some(data.characters.clone());
            }
            SubjectData::Kanji(ref data) => {
                characters_ = Some(data.characters.clone());
            }
            SubjectData::Vocabulary(ref data) => {
                characters_ = Some(data.characters.clone());
            }
            SubjectData::KanaVocabulary(ref data) => {
                characters_ = Some(data.characters.clone());
            }
        }

        let ascii_character = generate_ascii_text(&characters_.unwrap(), "togoshi-gothic", 120, 2);
        match ascii_character {
            Ok(ascii_character) => display::display_text(output_method, &ascii_character),
            Err(e) => display::display_text(output_method, &format!("Error: {}", e)),
        }
    }

    loop {
        let display_menu: &[(&char, &str, menu::MenuAction)] = menu::INTRO_MENU;

        let user_choice = display::display_menu(output_method, &display_menu);

        if let Some((_, _, action)) = display_menu.iter().find(|(key, _, _)| *key == &user_choice) {
            action(output_method, &client);
            if user_choice == '5' || user_choice == 'q' {
                break;
            }
        } else {
            println!("Invalid choice. Please try again.");
        }
    }
}
