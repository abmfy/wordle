use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::read_word_list;

pub const DEFAULT_DAY: u32 = 1;
pub const DEFAULT_SEED: u64 = 19260817;

/// Returns the successor of the identity element of addition
fn get_one() -> Option<u32> {
    Some(DEFAULT_DAY)
}

/// The birthday of you-know-who
fn get_default_seed() -> Option<u64> {
    Some(DEFAULT_SEED)
}

/// Command line arguments
#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author = "abmfy", about = "A Wordle game, refined")]
pub struct Args {
    /// Specify the answer
    #[clap(short, long)]
    pub word: Option<String>,

    /// Randomly choose the answer
    #[clap(short, long, conflicts_with = "word")]
    #[serde(default)]
    pub random: bool,

    /// Enter difficult mode, where you must guess according to the former result
    #[clap(short = 'D', long)]
    #[serde(default)]
    pub difficult: bool,

    /// Show statistics
    #[clap(short = 't', long)]
    #[serde(default)]
    pub stats: bool,

    /// Specify current day
    #[clap(short, long, conflicts_with = "word",
        value_parser=clap::value_parser!(u32).range(1..))
    ]
    #[serde(default = "get_one")]
    pub day: Option<u32>,

    /// Specify random seed
    #[clap(short, long, conflicts_with = "word")]
    #[serde(default = "get_default_seed")]
    pub seed: Option<u64>,

    /// Specify the final answer list
    #[clap(short, long, value_parser = is_valid_word_list, value_name = "FILE")]
    pub final_set: Option<PathBuf>,

    /// Specify the acceptable word list
    #[clap(short, long, value_parser = is_valid_word_list, value_name = "FILE")]
    pub acceptable_set: Option<PathBuf>,

    /// Enable state saving and specify save file
    #[clap(short = 'S', long, value_name = "FILE")]
    pub state: Option<PathBuf>,

    /// Specify default parameters from a JSON file
    #[clap(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

impl Args {
    pub fn load_defaults(path: &Path) -> Result<Args, ()> {
        let mut contents = String::new();
        if let Ok(mut file) = File::open(path) {
            file.read_to_string(&mut contents).unwrap();
        } else {
            return Err(());
        }
        let args = serde_json::from_str::<Args>(contents.as_str());
        if let Ok(args) = args {
            Ok(args)
        } else {
            Err(())
        }
    }

    pub fn validate_word_list(&self) -> Result<(), String> {
        if self.final_set.is_some() {
            if let Err(message) =
                is_valid_word_list(self.final_set.as_ref().unwrap().to_str().unwrap())
            {
                return Err(message);
            }
        }
        if self.acceptable_set.is_some() {
            if let Err(message) =
                is_valid_word_list(self.acceptable_set.as_ref().unwrap().to_str().unwrap())
            {
                return Err(message);
            }
        }
        Ok(())
    }

    pub fn validate(&self, answer_list: &Vec<String>) -> Result<(), String> {
        if let Some(day) = self.day {
            if day == 0 {
                return Err("Day must be greater than 0!".to_string());
            }
            if day > answer_list.len() as u32 {
                return Err(
                    "Day should be less than or equal to the number of answers!".to_string()
                );
            }
        }
        if let Some(ref word) = self.word {
            if !answer_list.contains(&word.to_lowercase()) {
                return Err("Provided answer is not in the answer words list!".to_string());
            }
        }
        // Conflicting arguments
        if self.word.is_some() && self.random {
            return Err("Conflicting arguments: --word and --random".to_string());
        }
        // Depending arguments
        if self.seed.is_some() && !self.random {
            return Err("--seed requires --random".to_string());
        }
        if self.day.is_some() && !self.random {
            return Err("--day requires --random".to_string());
        }

        Ok(())
    }
}

/// Check if a word list file is in valid format, and store the words
fn is_valid_word_list(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    // Check if the file exists
    if !path.exists() {
        return Err("File does not exist".to_string());
    } else if !path.is_file() {
        return Err("Not a file".to_string());
    }

    let mut file = File::open(path.as_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Check if the file is in valid format
    let list = read_word_list(&path);
    // Check if the words are in length 5, and consists of only latin letters
    if !list
        .iter()
        .all(|word| word.len() == 5 && word.chars().all(|c| c.is_ascii_alphabetic()))
    {
        return Err("Invalid word list: words should consist of 5 latin letters".to_string());
    };

    if list.len() == 0 {
        return Err("Invalid word list: empty file".to_string());
    }

    Ok(path)
}
