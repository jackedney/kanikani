use std::io::{self, Write};

fn clear_lines(num_lines: u16) {
    print!("\x1B[{}A", num_lines); // Move the cursor up by num_lines
    for _ in 0..num_lines {
        print!("\r\x1B[K"); // Move the cursor to the beginning of the line and clear it
        println!();
    }
    print!("\x1B[{}A", num_lines); // Move the cursor back up to the original position
    io::stdout().flush().unwrap();
}

pub fn display_start_screen(ascii_intro: &str) {
    println!("{}", ascii_intro);
    println!("---------------------");
    println!("Welcome to KaniKani!");
    println!("---------------------");
}

pub fn text_input(prompt: &str) -> String {
    print!("{}\n", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn display_menu(options: &[(&char, &str)]) -> char {
    println!("\nMain Menu:");

    for (key, option) in options {
        println!("{}. {}", key, option);
    }
    println!("\nEnter your choice:");

    loop {
        let input = text_input("");
        match input.parse::<char>() {
            Ok(choice) if options.iter().any(|(k, _)| k == &&choice) => {
                clear_lines((options.len() + 4) as u16);
                return choice;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

pub fn display_text(text: &str) {
    println!("{}", text);
}
