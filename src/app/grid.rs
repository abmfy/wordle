use crate::game::{Game, LetterStatus};

use super::{
    letter::{letter, Letter},
    metrics,
};

/// Render the letter grid
pub fn grid(ui: &mut egui::Ui, game: &Game, guess: &mut String, difficult: bool) {
    // The letter grid
    for i in 0..metrics::ROWS as usize {
        for j in 0..metrics::COLUMNS as usize {
            // Already guessed
            if i < game.get_round() {
                let guess = &game.get_guesses()[i];
                let letter_char = Some(guess.0.chars().nth(j).unwrap().to_ascii_uppercase());
                let status = guess.1[j];
                letter(
                    ui,
                    difficult,
                    i as i32,
                    j as i32,
                    &Letter {
                        letter: letter_char,
                        status,
                    },
                );
            } else if i == game.get_round() && j < guess.len() {
                // We'll input words in this row, and the jth letter already input
                letter(
                    ui,
                    difficult,
                    i as i32,
                    j as i32,
                    &Letter {
                        letter: guess.chars().nth(j),
                        status: LetterStatus::Unknown,
                    },
                )
            } else {
                // Blank letter
                letter(
                    ui,
                    difficult,
                    i as i32,
                    j as i32,
                    &Letter {
                        letter: None,
                        status: LetterStatus::Unknown,
                    },
                );
            }
        }
    }
}
