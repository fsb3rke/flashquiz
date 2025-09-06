mod fs_utils;
mod models;
mod flashcard;
mod quiz;

use std::path::Path;

fn main() -> anyhow::Result<()> {
    let dir = Path::new("./flashcards");
    let files = fs_utils::list_json_files(dir)?;

    if files.is_empty() {
        println!("There's is no JSOn file in this directory");
        return Ok(());
    }

    for file in files {
        println!("\n==> File: {}", file);
        let path = Path::new(&file);
        let flashcards = fs_utils::read_flashcard_file(path)?;

        flashcard::show_flashcards(&flashcards);

        quiz::start_quiz(&flashcards);
    }

    Ok(())
}
