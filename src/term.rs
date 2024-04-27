use std::io::{self, Write};

pub fn display_start_screen() {
    println!("Welcome to KaniKani!");
    println!("---------------------");
}

pub fn text_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn display_menu(options: &[&str]) -> usize {
    println!("\nMain Menu:");
    for (index, option) in options.iter().enumerate() {
        println!("{}. {}", index + 1, option);
    }
    println!("\nEnter your choice:");

    loop {
        let input = text_input("");
        match input.parse::<usize>() {
            Ok(choice) if choice > 0 && choice <= options.len() => return choice,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

pub fn display_text(text: &str) {
    println!("{}", text);
}
