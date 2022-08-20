use std::collections::HashMap;

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
}

impl Stats {
    /// Initialize statistics from scratch or JSON file
    pub fn new(use_state: bool) -> Option<Self> {
        if use_state {
            None
        } else {
            Some(Self {
                wins: 0,
                fails: 0,
                tries: 0,
                word_usage: Counter::new(),
            })
        }
    }

    /// Won a game, update stats
    pub fn win(&mut self, guesses: &Vec<(String, GuessStatus)>) {
        self.wins += 1;
        self.tries += guesses.len() as i32;
        for (word, _) in guesses {
            count(&mut self.word_usage, word.to_string());
        }
    }

    /// Failed a game, update stats
    pub fn fail(&mut self, guesses: &Vec<(String, GuessStatus)>) {
        self.fails += 1;
        for (word, _) in guesses {
            count(&mut self.word_usage, word.to_string());
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
