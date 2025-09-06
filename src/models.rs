use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FlashcardList {
    pub cards: Vec<HashMap<String, String>>,
}
