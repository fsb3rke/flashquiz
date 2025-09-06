mod fs_utils;
mod models;
mod flashcard;
mod quiz;

use std::path::Path;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let dir = Path::new("./flashcards");
    let files = fs_utils::list_json_files(dir)?;

    if files.is_empty() {
        println!("There's is no JSOn file in this directory");
        return Ok(());
    }

    println!("Existed flashcard files:");
    for (i, file) in files.iter().enumerate() {
        println!("\t[{}] {}", i+1, file);
    }

    print!("Select flashcard file (index)");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice == 0 || choice > files.len() {
        println!("Invalid selection");
        return Ok(());
    }

    let file = &files[choice-1];
    println!("\n==> Selected File: {}", file);

    let path = Path::new(file);
    let flashcards = fs_utils::read_flashcard_file(path)?;

    flashcard::show_flashcards(&flashcards);
    quiz::start_quiz(&flashcards);

    Ok(())
}
