use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, Clear, ClearType},
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;
use std::io;

pub fn display_start_screen(ascii_intro: &str) {
    let mut terminal = setup_terminal().expect("Failed to setup terminal");
    execute!(terminal.backend_mut(), Clear(ClearType::All)).unwrap();
    terminal
        .draw(|f| {
            let block = Block::default().title(ascii_intro).borders(Borders::ALL);
            f.render_widget(block, f.size());
        })
        .expect("Failed to draw start screen");
    wait_for_key_press();
    terminal.clear().expect("Failed to clear terminal");
}

pub fn text_input(prompt: &str) -> String {
    let mut terminal = setup_terminal().expect("Failed to setup terminal");
    let mut input = String::new();
    terminal
        .draw(|f| {
            let prompt_widget = Paragraph::new(prompt);
            let input_widget = Paragraph::new(&*input).style(Style::default());
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
                .split(f.size());
            f.render_widget(prompt_widget, layout[0]);
            f.render_widget(input_widget, layout[1]);
        })
        .expect("Failed to draw input prompt");
    loop {
        if let Event::Key(key) = event::read().expect("Failed to read event") {
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
            terminal
                .draw(|f| {
                    let input_widget = Paragraph::new(&*input).style(Style::default());
                    f.render_widget(input_widget, f.size());
                })
                .expect("Failed to draw input");
        }
    }
    terminal.clear().expect("Failed to clear terminal");
    input
}

pub fn display_menu(options: &[(&char, &str)]) -> char {
    let mut terminal = setup_terminal().expect("Failed to setup terminal");
    let mut selected_index = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    loop {
        terminal
            .draw(|f| {
                let items: Vec<ListItem> = options
                    .iter()
                    .map(|(k, v)| ListItem::new(format!("{}. {}", k, v)))
                    .collect();
                let list = List::new(items)
                    .block(Block::default().title("Main Menu").borders(Borders::ALL))
                    .style(Style::default())
                    .highlight_style(
                        Style::default()
                            .bg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">>");
                f.render_stateful_widget(list, f.size(), &mut list_state);
            })
            .expect("Failed to draw menu");

        if let Event::Key(key) = event::read().expect("Failed to read event") {
            match key.code {
                KeyCode::Enter => {
                    let selected_choice = options[selected_index].0;
                    return *selected_choice;
                }
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                        list_state.select(Some(selected_index));
                    }
                }
                KeyCode::Down => {
                    if selected_index < options.len() - 1 {
                        selected_index += 1;
                        list_state.select(Some(selected_index));
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn display_text(text: &str) {
    let mut terminal = setup_terminal().expect("Failed to setup terminal");
    terminal
        .draw(|f| {
            let paragraph = Paragraph::new(text);
            f.render_widget(paragraph, f.size());
        })
        .expect("Failed to draw text");
    wait_for_key_press();
    terminal.clear().expect("Failed to clear terminal");
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn wait_for_key_press() {
    loop {
        if let Event::Key(_) = event::read().expect("Failed to read event") {
            break;
        }
    }
}
