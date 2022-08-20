use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;

use super::read_word_list;

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author = "abmfy", about = "A Wordle game, refined")]
pub struct Args {
    /// Specify the answer
    #[clap(short, long)]
    pub word: Option<String>,

    /// Randomly choose the answer
    #[clap(short, long, conflicts_with = "word")]
    pub random: bool,

    /// Enter difficult mode, where you must guess according to the former result
    #[clap(short = 'D', long)]
    pub difficult: bool,

    /// Show statistics
    #[clap(short = 't', long)]
    pub stats: bool,

    /// Specify current day
    #[clap(short, long, requires = "random", conflicts_with = "word", default_value_t = 1,
        value_parser=clap::value_parser!(u32).range(1..))
    ]
    pub day: u32,

    /// Specify random seed
    #[clap(
        short,
        long,
        requires = "random",
        conflicts_with = "word",
        default_value_t = 19260817
    )]
    pub seed: u64,

    /// Specify the final answer list
    #[clap(short, long, value_parser = is_valid_word_list, value_name = "FILE")]
    pub final_set: Option<PathBuf>,

    /// Specify the acceptable word list
    #[clap(short, long, value_parser = is_valid_word_list, value_name = "FILE")]
    pub acceptable_set: Option<PathBuf>,

    /// Whether to save and load state
    #[clap(short, long)]
    pub state: bool,
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
