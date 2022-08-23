use std::collections::HashMap;

use console::Color;
use console::StyledObject;
use serde::Deserialize;
use serde::Serialize;

const ALPHABET_SIZE: usize = 26;
const WORD_LENGTH: usize = 5;
const MAX_GAME_ROUND: usize = 6;

#[derive(Debug)]
pub enum Error {
    UnexpectedWordLength,
    UnknownWord,
    BadAnswer,
    HintUnused,
}

impl Error {
    /// Get what happened
    pub fn what(&self) -> String {
        match self {
            Self::UnexpectedWordLength => format!("The length of a word should be {WORD_LENGTH}."),
            Self::UnknownWord => String::from("Unknown word, please try again."),
            Self::BadAnswer => {
                String::from("That seems not suitable for a Wordle game. Maybe pick another?")
            }
            Self::HintUnused => String::from("You must use the hint in difficult mode."),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LetterStatus {
    Unknown,
    Red,
    Yellow,
    Green,
}

impl LetterStatus {
    pub fn to_char(&self) -> char {
        match self {
            Self::Unknown => 'X',
            Self::Red => 'R',
            Self::Yellow => 'Y',
            Self::Green => 'G',
        }
    }

    // Render letter c with the color of this status
    pub fn colored_char(&self, c: char) -> StyledObject<char> {
        console::style(c).fg(match self {
            // Gray
            Self::Unknown => Color::Color256(102),
            Self::Red => Color::Red,
            Self::Yellow => Color::Yellow,
            Self::Green => Color::Green,
        })
    }
}

pub type GuessStatus = [LetterStatus; WORD_LENGTH];
pub type Alphabet = [LetterStatus; ALPHABET_SIZE];

// Get the index of a letter in an alphabet
pub fn get_index(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Going,
    Won(usize),
    Failed(String),
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    answer: String,
    guesses: Vec<(String, GuessStatus)>,
    alphabet: Alphabet,
    difficult: bool,
}

impl Game {
    /// Start a new game with given answer
    pub fn new(answer: &str, difficult: bool, answer_list: &Vec<String>) -> Result<Self, Error> {
        // Provided answer not in good answer list
        if !answer_list.contains(&answer.to_string()) {
            return Err(Error::BadAnswer);
        }
        Ok(Self {
            answer: answer.to_string(),
            guesses: vec![],
            alphabet: [LetterStatus::Unknown; ALPHABET_SIZE],
            difficult,
        })
    }

    /// How many rounds has this game gone through
    pub fn get_round(&self) -> usize {
        self.guesses.len()
    }

    // Getter for guesses
    pub fn get_guesses(&self) -> &Vec<(String, GuessStatus)> {
        &self.guesses
    }

    // Getter for alphabet
    pub fn get_alphabet(&self) -> &Alphabet {
        &&self.alphabet
    }

    /// Get the status of a guess
    fn get_guess_status(&self, word: &str) -> Result<GuessStatus, Error> {
        // Auxiliary type and function for counting occurrence of letters
        type Counter = HashMap<char, usize>;
        fn count(counter: &mut Counter, letter: char) -> usize {
            *counter
                .entry(letter)
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1)
        }

        // Count occurrence of letters in the answer
        let mut ans_counter = Counter::new();
        self.answer.chars().for_each(|c| {
            count(&mut ans_counter, c);
        });

        // Difficult mode check
        if self.difficult && self.get_round() > 0 {
            let (last_guess, last_status) = self.guesses.last().unwrap();

            let mut guess_counter = Counter::new();
            word.chars().for_each(|c| {
                count(&mut guess_counter, c);
            });

            // Count the occurrence of yellow and green letters for check
            let mut last_guess_counter = Counter::new();

            for ((i, last_letter), now_letter) in last_guess.chars().enumerate().zip(word.chars()) {
                match last_status[i] {
                    LetterStatus::Green => {
                        // Green letters must stay green
                        if now_letter != last_letter {
                            return Err(Error::HintUnused);
                        }
                        count(&mut last_guess_counter, last_letter);
                    }
                    LetterStatus::Yellow => {
                        count(&mut last_guess_counter, last_letter);
                    }
                    _ => (),
                }
            }

            for (letter, count) in &last_guess_counter {
                if guess_counter.get(letter).unwrap_or(&0) < count {
                    return Err(Error::HintUnused);
                }
            }
        }

        let mut result = [LetterStatus::Unknown; WORD_LENGTH];

        // Firstly go through the guess to match correct letters
        let mut visited = [false; 5];
        for (i, c) in word.chars().enumerate() {
            if self.answer.chars().nth(i).unwrap() == c {
                visited[i] = true;
                // This letter is matched, so decrement the count in answer counter
                // is order that it won't be matched again
                *ans_counter.get_mut(&c).unwrap() -= 1;
            }
        }

        let mut guess_counter = Counter::new();
        word.chars().enumerate().for_each(|(i, c)| {
            // Increment the occurrence count of current letter, and compare it with the one in answer
            result[i] = if visited[i] {
                LetterStatus::Green
            } else if count(&mut guess_counter, c) <= *ans_counter.get(&c).unwrap_or(&0) {
                LetterStatus::Yellow
            } else {
                LetterStatus::Red
            };
        });
        Ok(result)
    }

    /// Update the alphabet based on the result of a guess
    fn update_alphabet(&mut self, word: &str, status: &GuessStatus) {
        for (i, c) in word.chars().enumerate() {
            let index = get_index(c);
            // Update the state of the letters in the word
            self.alphabet[index] = self.alphabet[index].max(status[i]);
        }
    }

    /// Make a guess
    pub fn guess(&mut self, word: &str, word_list: &Vec<String>) -> Result<GameStatus, Error> {
        if word.len() != WORD_LENGTH {
            return Err(Error::UnexpectedWordLength);
        }
        // Word not in acceptable word list
        if word_list.binary_search(&word.to_string()).is_err() {
            return Err(Error::UnknownWord);
        }

        let guess_status = self.get_guess_status(word)?;
        self.update_alphabet(word, &guess_status);
        self.guesses.push((word.to_string(), guess_status));

        const COMPLETE_STATUS: GuessStatus = [LetterStatus::Green; 5];

        let round = self.get_round();
        let game_status = if guess_status == COMPLETE_STATUS {
            GameStatus::Won(round)
        } else if round == MAX_GAME_ROUND {
            GameStatus::Failed(self.answer.to_string())
        } else {
            GameStatus::Going
        };

        Ok(game_status)
    }
}
