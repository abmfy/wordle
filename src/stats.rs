use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;

use super::game::GuessStatus;

/// Counter for counting words usage
type Counter = HashMap<String, usize>;
fn count(counter: &mut Counter, word: String) -> usize {
    *counter.entry(word).and_modify(|cnt| *cnt += 1).or_insert(1)
}

pub struct Stats {
    wins: i32,
    fails: i32,
    tries: i32,
    word_usage: Counter,
    games: Vec<Game>,
    state_path: Option<PathBuf>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Game {
    answer: String,
    guesses: Vec<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    total_rounds: Option<u32>,
    games: Option<Vec<Game>>,
}

impl Stats {
    /// Return an initial state of stats
    fn default() -> Self {
        Self {
            wins: 0,
            fails: 0,
            tries: 0,
            word_usage: Counter::new(),
            games: vec![],
            state_path: None,
        }
    }

    /// Initialize statistics from scratch or from JSON file.
    /// Return None if 'state.json' is in invalid format
    pub fn new(state_path: &Option<PathBuf>) -> Option<Self> {
        let use_state = state_path.is_some();
        // Use state mode
        if use_state {
            // If the state file exists, read it
            if PathBuf::from(state_path.as_ref().unwrap()).exists() {
                if let Ok(state) = serde_json::from_str::<State>(
                    fs::read_to_string(state_path.as_ref().unwrap())
                        .unwrap()
                        .as_str(),
                ) {
                    let mut stats = Self::default();
                    stats.state_path = state_path.clone();

                    // Load stats from file
                    if let Some(games) = state.games {
                        for game in games {
                            stats.games.push(game.clone());
                            if game.guesses.last()? == &game.answer {
                                stats.wins += 1;
                                stats.tries += game.guesses.len() as i32;
                            } else {
                                stats.fails += 1;
                            }
                            for word in game.guesses {
                                count(&mut stats.word_usage, word.to_lowercase());
                            }
                        }
                    }
                    Some(stats)
                } else {
                    None
                }
            } else {
                // Create new state file
                let mut stats = Self::default();
                stats.state_path = state_path.clone();
                Some(stats)
            }
        } else {
            Some(Self::default())
        }
    }

    pub fn save(&mut self) {
        let state = State {
            total_rounds: Some((self.wins + self.fails) as u32),
            games: Some(self.games.clone()),
        };
        fs::write(self.state_path.as_ref().unwrap(), json!(state).to_string()).unwrap();
    }

    fn update_guesses(&mut self, guesses: &Vec<(String, GuessStatus)>, answer: &String) {
        let mut words: Vec<String> = vec![];
        for (word, _) in guesses {
            count(&mut self.word_usage, word.to_string());
            words.push(word.to_uppercase());
        }
        self.games.push(Game {
            answer: answer.to_uppercase(),
            guesses: words,
        })
    }

    /// Won a game, update stats
    pub fn win(&mut self, save: bool, guesses: &Vec<(String, GuessStatus)>) {
        self.wins += 1;
        self.tries += guesses.len() as i32;
        self.update_guesses(guesses, &guesses.last().unwrap().0);
        if save {
            self.save();
        }
    }

    /// Failed a game, update stats
    pub fn fail(&mut self, save: bool, guesses: &Vec<(String, GuessStatus)>, answer: &String) {
        self.fails += 1;
        self.update_guesses(guesses, answer);
        if save {
            self.save();
        }
    }

    /// Print statistics in tty mode
    pub fn print(&self, is_tty: bool) {
        let average_tries = if self.wins == 0 {
            0.0
        } else {
            self.tries as f64 / self.wins as f64
        };

        // Sort used words by usage times
        let mut vec: Vec<(&String, &usize)> = self.word_usage.iter().collect();
        vec.sort_by(|(word1, cnt1), (word2, cnt2)| {
            if cnt1 != cnt2 {
                return cnt1.cmp(cnt2);
            }
            return word1.cmp(word2).reverse();
        });

        if is_tty {
            println!("{}", console::style("Statistics:").bold().yellow());
            println!(
                "{} {} {} {}",
                console::style("Wins:").bold().green(),
                self.wins,
                console::style("Fails:").bold().red(),
                self.fails,
            );
            println!(
                "{} {average_tries:.2}",
                console::style("Average tries of games won:").bold()
            );
            println!(
                "{}",
                console::style("Most frequently used words:").bold().blue()
            );
            for (word, count) in vec.iter().rev().take(5) {
                println!(
                    "    {}: used {count} times ",
                    console::style(word.to_uppercase()).bold().magenta()
                );
            }
        } else {
            println!("{} {} {average_tries:.2}", self.wins, self.fails);

            let mut first = true;
            for (word, count) in vec.iter().rev().take(5) {
                if !first {
                    print!(" ");
                }
                first = false;
                print!("{} {count}", word.to_uppercase());
            }
            println!("");
        }
    }
}
