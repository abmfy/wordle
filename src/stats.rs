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

#[derive(Default, Serialize, Deserialize)]
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
                                count(&mut stats.word_usage, word);
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

    /// Getter for wins
    pub fn get_wins(&self) -> i32 {
        self.wins
    }

    /// Getter for fails
    pub fn get_fails(&self) -> i32 {
        self.fails
    }

    /// Get average tries of game won
    pub fn get_average_tries(&self) -> f64 {
        if self.wins == 0 {
            0.0
        } else {
            self.tries as f64 / self.wins as f64
        }
    }

    /// Get favorite five words
    pub fn get_favorite_words(&self) -> Vec<(&String, &usize)> {
        // Sort used words by usage times
        let mut words: Vec<(&String, &usize)> = self.word_usage.iter().collect();
        words.sort_by(|(word1, cnt1), (word2, cnt2)| {
            if cnt1 != cnt2 {
                return cnt1.cmp(cnt2);
            }
            return word1.cmp(word2).reverse();
        });
        words.iter().cloned().rev().take(5).collect()
    }

    /// Save stats to specified path
    pub fn save(&mut self) {
        let state = State {
            total_rounds: Some((self.wins + self.fails) as u32),
            games: Some(self.games.clone()),
        };
        fs::write(self.state_path.as_ref().unwrap(), json!(state).to_string()).unwrap();
    }

    /// Update the stats of a single guess
    pub fn update_guess(&mut self, guess: &str) {
        count(&mut self.word_usage, guess.to_string());
    }

    /// Update stats of the guesses
    fn update_guesses(&mut self, guesses: &Vec<(String, GuessStatus)>, answer: &String) {
        let mut words: Vec<String> = vec![];
        for (word, _) in guesses {
            self.update_guess(word);
            words.push(word.to_string());
        }
        self.games.push(Game {
            answer: answer.to_string(),
            guesses: words,
        })
    }

    /// Won a game with given round, update stats
    /// This function is here for GUI. In GUI mode we update guess stats every guess,
    /// so we don't need to update guess stats again when game is over
    pub fn win_with_guesses_updated(&mut self, round: usize) {
        self.wins += 1;
        self.tries += round as i32;
    }

    /// Won a game, update stats
    pub fn win(&mut self, save: bool, guesses: &Vec<(String, GuessStatus)>) {
        self.win_with_guesses_updated(guesses.len());
        self.update_guesses(guesses, &guesses.last().unwrap().0);
        if save {
            self.save();
        }
    }

    /// Failed a game, update stats
    /// This function is here for GUI. In GUI mode we update guess stats every guess,
    /// so we don't need to update guess stats again when game is over
    pub fn fail_with_guesses_updated(&mut self) {
        self.fails += 1;
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
        let average_tries = self.get_average_tries();

        // Sort used words by usage times
        let words = self.get_favorite_words();

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
            for (word, count) in words {
                println!(
                    "    {}: used {count} times ",
                    console::style(word).bold().magenta()
                );
            }
        } else {
            println!("{} {} {average_tries:.2}", self.wins, self.fails);

            let mut first = true;
            for (word, count) in words {
                if !first {
                    print!(" ");
                }
                first = false;
                print!("{word} {count}");
            }
            println!("");
        }
    }
}
