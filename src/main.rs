
use std::fs;

static LOGO_ART_PATH: &str = "src/art/kanilogo.txt";
static NAME_ART_PATH: &str = "src/art/kaniname.txt";

fn main() {
    let logo_art = fs::read_to_string(LOGO_ART_PATH).expect("Failed to read wanilogo.txt file");
    let name_art = fs::read_to_string(NAME_ART_PATH).expect("Failed to read waniname.txt file");

    // Print the ASCII art sequentially
    println!("{}", logo_art);
    println!("{}", name_art);

    // print line across the terminal
    println!("{}", "-".repeat(80));
    println!("ようこそ！ Welcome to kanikani - the CLI tool for doing your WaniKani reviews!");
}
