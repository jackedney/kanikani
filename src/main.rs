mod config;
mod wanikani;

use crate::config::{load_config, save_config, Config};
use crate::wanikani::create_client;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Margin;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::fs;
use terminal_size::{terminal_size, Height, Width};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

mod ui {
    use super::*;

    static LOGO_ART_PATH: &str = "src/art/kanilogo.txt";
    static NAME_ART_PATH: &str = "src/art/kaniname.txt";

    pub fn setup_terminal(
    ) -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, Box<dyn std::error::Error>> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(terminal)
    }

    pub fn create_layout(
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<Vec<ratatui::layout::Rect>, Box<dyn std::error::Error>> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(terminal.size()?)
            .to_vec();
        Ok(chunks)
    }

    pub fn create_logo_widget<'a>() -> Paragraph<'a> {
        if let Some((Width(w), Height(_))) = terminal_size() {
            let logo_art =
                fs::read_to_string(LOGO_ART_PATH).expect("Failed to read wanilogo.txt file");
            let name_art =
                fs::read_to_string(NAME_ART_PATH).expect("Failed to read waniname.txt file");
            let full_art = format!("{}{}", logo_art, name_art);
            let lines: Vec<&str> = full_art.split('\n').collect();

            // Create a new string to store the padded ASCII art
            let mut padded_ascii_art = String::new();

            // Add padding to each line of the ASCII art
            for line in lines {
                let padded_line = format!("{:^width$}\n", line, width = w as usize);
                padded_ascii_art.push_str(&padded_line);
            }
            Paragraph::new(padded_ascii_art)
                .block(Block::default())
                .alignment(Alignment::Center)
        } else {
            Paragraph::new("Failed to get terminal size")
                .block(Block::default())
                .alignment(Alignment::Center)
        }
    }

    pub fn render_ui(
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        chunks: &[ratatui::layout::Rect],
        logo_widget: Paragraph,
        welcome_msg: &str,
        input_msg: &str,
        input: &Input,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|f| {
            let welcome_widget = Paragraph::new(welcome_msg)
                .block(Block::default().title("kanikani").borders(Borders::ALL));
            f.render_widget(logo_widget.clone(), chunks[0]);
            f.render_widget(welcome_widget, chunks[1]);

            let masked_input = "*".repeat(input.value().len());
            let input_widget = Paragraph::new(masked_input)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().title(input_msg));
            let input_area = chunks[1].inner(&Margin {
                vertical: 3,
                horizontal: 1,
            });
            f.render_widget(input_widget, input_area);
            f.set_cursor(
                input_area.x + input.visual_cursor() as u16 + 1,
                input_area.y + 1,
            );

            // Render the "(q) to quit" message at the bottom
            let quit_message = Paragraph::new("(q) to quit")
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Right);
            let quit_area = chunks[1].inner(&Margin {
                vertical: 1,
                horizontal: 1,
            });
            f.render_widget(quit_message, quit_area);
        })?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ui::setup_terminal()?;
    let chunks = ui::create_layout(&mut terminal)?;
    let logo_widget = ui::create_logo_widget();

    let welcome_msg =
        "ようこそ！ Welcome to kanikani - the CLI tool for doing your WaniKani reviews!\n";
    let mut input = Input::default();

    let api_token = match load_config() {
        Some(config) => config.api_token,
        None => {
            let input_msg = "Please enter your WaniKani API token:";

            loop {
                ui::render_ui(
                    &mut terminal,
                    &chunks,
                    logo_widget.clone(),
                    welcome_msg,
                    &input_msg,
                    &input,
                )?;

                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    match key.code {
                        crossterm::event::KeyCode::Enter => {
                            let api_token = input.value().to_string();
                            save_config(&Config {
                                api_token: api_token.clone(),
                            });
                            break api_token;
                        }
                        _ => {
                            input.handle_event(&crossterm::event::Event::Key(key));
                        }
                    }
                }
            }
        }
    };

    // Set the API token as an environment variable
    std::env::set_var("WANIKANI_API_TOKEN", api_token);

    match create_client().await {
        Ok(_client) => {
            println!("\nAuthentication successful!");
            // Authentication successful, proceed with the application
            // ...
        }
        Err(e) => {
            // Authentication failed, display an error message
            let error_msg = format!("Authentication failed: {}", e);
            ui::render_ui(
                &mut terminal,
                &chunks,
                logo_widget,
                &error_msg,
                "",
                &Input::default(),
            )?;
            // You can choose to exit the application or handle the error differently
            return Ok(());
        }
    }
    loop {
        let mut quit = false;

        match create_client().await {
            Ok(_client) => {
                ui::render_ui(
                    &mut terminal,
                    &chunks,
                    logo_widget.clone(),
                    "",
                    "",
                    &Input::default(),
                )?;
            }
            Err(_e) => {
                ui::render_ui(
                    &mut terminal,
                    &chunks,
                    logo_widget.clone(),
                    "",
                    "",
                    &Input::default(),
                )?;
            }
        }

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if let crossterm::event::KeyCode::Char('q') = key.code {
                quit = true;
            }
        }

        if quit {
            break;
        }
    }

    Ok(())
}
