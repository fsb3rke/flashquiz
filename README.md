# flashquiz

`flashquiz` is a simple Rust CLI application that lets you practice flashcards stored in JSON files. It first displays all cards in a selected file, then switches to quiz mode, asking random questions. Instant feedback is given with ✅ for correct answers and ❌ for wrong ones.

## Features

- List all JSON flashcard files in a folder
- Display all cards before quiz
- Quiz mode with random order questions
- Instant feedback for answers
- Per-file error handling to avoid crashes

## Installation

Make sure Rust is installed. Clone the repository and build:

git clone <your-repo-url>
cd flashquiz
cargo build --release

## Usage

1. Create a `flashcards/` folder in the project root.
2. Add JSON files like this:

{
  "cards": [
    {"A": "Alpha"},
    {"B": "Bravo"},
    {"C": "Charlie"}
  ]
}

3. Run the application:

cargo run

4. Select a flashcard file by entering its number.
5. Cards are displayed first, then quiz starts.

## Example

📂 Available flashcard files:
  [1] flashcards/nato.json
  [2] flashcards/animals.json
Select a file (number): 1

==> Selected file: flashcards/nato.json
📚 Flashcards:
  A → Alpha
  B → Bravo
  C → Charlie

🎯 Quiz starts! (Press Enter to exit)
❓ A → Alpha
✅ Correct!
❓ B → Brvo
❌ Wrong! Correct answer: Bravo
❓ C → 
🚪 Exited quiz.

## License

This project is licensed under the **GNU General Public License v3.0 (GPLv3)**. See the [LICENSE](https://github.com/fsb3rke/flashquiz/blob/main/LICENSE) file for details.
