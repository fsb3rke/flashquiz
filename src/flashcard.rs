use crate::models::FlashcardList;

pub fn show_flashcards(list: &FlashcardList) {
    println!("Flashcards:");
    for card in &list.cards {
        for (key, value) in card {
            println!("\t{} -> {}", key, value);
        }
    }
}
