use std::fs;
use std::path::Path;
use crate::models::FlashcardList;

pub fn read_flashcard_file(path: &Path) -> anyhow::Result<FlashcardList> {
    let data = fs::read_to_string(path)?;
    let flashcards: FlashcardList = serde_json::from_str(&data)?;
    Ok(flashcards)
}

pub fn list_json_files(dir: &Path) -> anyhow::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Some(name) = path.to_str() {
                files.push(name.to_string());
            }
        }
    }
    Ok(files)
}
