use crate::models::FlashcardList;
use rand::seq::SliceRandom;
use std::io::{self, Write};

pub fn start_quiz(list: &FlashcardList) {
    println!("\nQuiz Starting (Press enter to exit)");

    let mut rng = rand::thread_rng();
    let mut cards = list.cards.clone();

    cards.shuffle(&mut rng);

    for card in cards {
        for (key, value) in &card {
            print!("{} -> ", key);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let answer = input.trim();

            if answer.is_empty() {
                println!("Exit");
                return;
            }

            if answer.eq_ignore_ascii_case(value) {
                println!("True!");
            } else {
                println!("Wrong: True answer is: {}", value);
            }
        }
    }
}
