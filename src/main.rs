mod config;
mod term;
mod tui;
mod wanikani;

use crate::config::{load_config, save_config, Config};
use crate::wanikani::api::WaniKaniClient;
use serde_json::to_string_pretty;

mod display {
    use crate::term;
    use crate::tui;

    pub fn display_start_screen(output_method: &str) {
        match output_method {
            "term" => term::display_start_screen(),
            "tui" => tui::display_start_screen(),
            _ => panic!("Invalid output method"),
        }
    }

    pub fn display_menu(output_method: &str, choices: &[&str]) -> usize {
        match output_method {
            "term" => term::display_menu(choices),
            "tui" => tui::display_menu(choices),
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

    let user_info = client.fetch_user_info().await;

    if let Err(e) = user_info {
        display::display_text(
            output_method,
            &String::from(format!("Failed to get user information: {}", e)),
        );
        return;
    } else if let Ok(user_info) = user_info {
        display::display_text(output_method, &to_string_pretty(&user_info).unwrap());
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main2() {
    // Determine the output method (term or tui)
    let output_method = "term"; // or "term"

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
        eprintln!("Authentication failed: {}", e);
        return;
    }

    // Main loop
    loop {
        let display_menu = [
            "Reviews",
            "Lessons",
            "Stats",
            "Dictionary",
            "Settings",
            "Logout",
            "Quit",
        ];
        // Display the main menu and get user input
        let user_choice = display::display_menu(output_method, &display_menu);

        match user_choice {
            1 => {
                display::display_text(output_method, "Reviews");
            }
            2 => {
                display::display_text(output_method, "Lessons");
            }
            3 => {
                display::display_text(output_method, "Stats");
            }
            4 => {
                display::display_text(output_method, "Dictionary");
            }
            5 => {
                display::display_text(output_method, "Settings");
            }
            6 => {
                println!("Logging out...");
                break;
            }
            7 => {
                println!("Quitting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
