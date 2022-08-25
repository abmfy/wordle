use clap::Parser;
use console;
use rand::{seq::SliceRandom, SeedableRng};
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process,
};

mod app;
mod args;
mod builtin_words;
mod dict;
mod game;
mod stats;

use app::WordleApp;
use args::Args;
use dict::DICT;
use game::{Error, Game, GameStatus, GuessStatus, LetterStatus};
use stats::Stats;

/// Read a line, trimmed. Return None if EOF encountered
fn read_line() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line.trim().to_string()),
    }
}

/// Read a word list from a file
fn read_word_list(path: &PathBuf) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split_whitespace()
        .map(|s| s.to_uppercase())
        .collect()
}

/// Flush the output
fn flush() {
    io::stdout().flush().unwrap();
}

/// Print an error message
fn print_error(is_tty: bool, error: &Error) {
    if is_tty {
        println!("{}", console::style(error.what()).bold().red());
    } else {
        println!("INVALID");
    }
}

/// Print status of letters, in non-tty mode
fn print_status(status: &[LetterStatus]) {
    print!("{}", String::from_iter(status.iter().map(|s| s.to_char())));
}

/// Print guess history, in tty mode
fn print_guess_history(guesses: &Vec<(String, GuessStatus)>) {
    for i in 0..6 {
        if i < guesses.len() {
            for (j, c) in guesses[i].0.chars().enumerate() {
                print!("{}", guesses[i].1[j].colored_char(c));
            }
            println!("");
        } else {
            println!("{}", console::style("_____").dim());
        }
    }
}

/// Print the alphabet, in tty mode
fn print_alphabet(alphabet: &[LetterStatus]) {
    const ROW1: &str = "QWERTYUIOP";
    const ROW2: &str = "ASDFGHJKL";
    const ROW3: &str = "ZXCVBNM";
    for row in [ROW1, ROW2, ROW3] {
        for c in row.chars() {
            print!("{}", alphabet[game::get_index(c)].colored_char(c));
        }
        println!("");
    }
}

/// Exit game normally and provide a message if in tty mode
fn exit_game(is_tty: bool) -> ! {
    if is_tty {
        println!("{}", console::style("Goodbye!").bold().green());
    }
    process::exit(0);
}

/// Exit game with error message and exit code 1
fn exit_with_error(is_tty: bool, message: &str) -> ! {
    if is_tty {
        println!("{}", console::style(message).bold().red());
    }
    process::exit(1);
}

/// The main function for the Wordle game, for native run
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use rand::Rng;

    let is_tty = atty::is(atty::Stream::Stdout);

    let mut args = Args::parse();

    // Start GUI
    if args.gui {
        eframe::run_native(
            "Wordle",
            eframe::NativeOptions::default(),
            Box::new(|cc| Box::new(WordleApp::new(cc))),
        );
        return;
    }

    // Config file specified
    if let Some(path) = args.config {
        // Load config file
        if let Ok(mut defaults) = Args::load_defaults(&path) {
            // Override config file with command line args
            defaults.update_from(env::args());
            args = defaults;
        } else {
            exit_with_error(is_tty, "Failed to load config file");
        }
    }

    // Validate word list first because we need it for validating other arguments
    if let Err(message) = args.validate_word_list() {
        exit_with_error(is_tty, &message);
    }

    // Current day
    let mut day = args.day.unwrap_or(args::DEFAULT_DAY) - 1;

    // Fetch acceptable words list
    let mut word_list: Vec<String> = if let Some(ref path) = args.acceptable_set {
        read_word_list(&path)
    } else {
        builtin_words::ACCEPTABLE
            .iter()
            .map(|s| s.to_uppercase())
            .collect()
    };

    // Sort the word list to accelerate search
    word_list.sort();

    // Fetch final words list
    let answer_list = {
        let mut list: Vec<String> = if let Some(ref path) = args.final_set {
            read_word_list(&path)
        } else {
            // If final words list not provided but acceptable list provided,
            // use the acceptable list as final words list
            if let Some(_) = args.acceptable_set {
                word_list.clone()
            } else {
                builtin_words::FINAL
                    .iter()
                    .map(|s| s.to_uppercase())
                    .collect()
            }
        };

        // Ensure the final words are a subset of acceptable words
        let final_set: HashSet<_> = list.iter().cloned().collect();
        let acceptable_set: HashSet<_> = word_list.iter().cloned().collect();
        if !final_set.is_subset(&acceptable_set) {
            exit_with_error(
                is_tty,
                "Final words should be a subset of acceptable words!",
            )
        }

        // When in random mode, shuffle the word list
        if args.random {
            let mut rng =
                rand::rngs::StdRng::seed_from_u64(args.seed.unwrap_or(args::DEFAULT_SEED));
            list.shuffle(&mut rng);
        }
        list
    };

    // Argument validation
    if let Err(message) = args.validate(&answer_list) {
        exit_with_error(is_tty, &message);
    };

    // Initiate statistics
    let mut stats = if let Some(stats) = Stats::new(&args.state) {
        stats
    } else {
        exit_with_error(
            is_tty,
            "Failed to load stats: 'state.json' broken\nYou should consider delete it.",
        );
    };

    // Print welcome message
    if is_tty {
        println!(
            "Welcome to {}{}{}{}{}{}!",
            console::style('W').bold().red(),
            console::style('o').bold().color256(208),
            console::style('r').bold().yellow(),
            console::style('d').bold().green(),
            console::style('l').bold().blue(),
            console::style('e').bold().color256(93),
        );

        println!("Note that you can type 'HINT' to get hints in the game!\n");

        let name = {
            print!(
                "{}",
                console::style("Could I have your name, please? ")
                    .bold()
                    .blue()
            );
            flush();
            let line = if let Some(line) = read_line() {
                line
            } else {
                exit_game(is_tty);
            };
            line.trim().to_string()
        };

        println!("Welcome, {}!\n", name);
    }

    // Game loop
    loop {
        // Did not provide answer
        let mut game = if args.word.is_none() {
            // Random mode
            if args.random {
                Game::new(&answer_list[day as usize], args.difficult, &answer_list).unwrap()
            } else {
                if is_tty {
                    print!(
                        "{}",
                        console::style("Please choose an answer for the game: ")
                            .bold()
                            .blue()
                    );
                    flush()
                }
                loop {
                    let answer: String = match read_line() {
                        Some(word) => word,
                        None => exit_game(is_tty),
                    };
                    let answer = answer.to_uppercase();
                    match Game::new(&answer, args.difficult, &answer_list) {
                        Ok(game) => break game,
                        Err(error) => print_error(is_tty, &error),
                    }
                }
            }
        } else {
            Game::new(
                &args.word.as_ref().unwrap().to_uppercase(),
                args.difficult,
                &answer_list,
            )
            .unwrap()
        };

        // Another day of playing wordle...
        // The mod is here to avoid overflow
        day += 1;
        day %= answer_list.len() as u32;

        loop {
            if is_tty {
                print!(
                    "{}",
                    console::style(format!("Guess {}: ", game.get_round() + 1)).blue()
                );
                flush();
            }

            let word: String = match read_line() {
                Some(word) => word,
                None => exit_game(is_tty),
            };
            let word = word.to_uppercase();

            // Get hint
            if word == "HINT" {
                let mut rng = rand::thread_rng();
                let hint = loop {
                    let index = rng.gen_range(0..word_list.len());
                    let word = &word_list[index];
                    if game.validate_guess(true, true, word, &word_list).is_ok() {
                        break word;
                    }
                };

                println!("{}", console::style(hint).bold().blue());
                continue;
            }

            let result = game.guess(&word, &word_list);
            match result {
                Ok(game_status) => {
                    let guesses = game.get_guesses();
                    let alphabet = game.get_alphabet();
                    // Print game status
                    if is_tty {
                        print_guess_history(guesses);
                        println!("--------------");
                        print_alphabet(alphabet);
                    } else {
                        print_status(&guesses.last().unwrap().1);
                        print!(" ");
                        print_status(alphabet);
                        println!("");
                    }

                    // If the word is in the dictionary, print its definition
                    let print_definition = |word| {
                        if dict::DICT.contains_key(word) {
                            println!("{}", console::style(format!("{word}:")).bold().blue());
                            for (i, sense) in DICT.get(word).unwrap().iter().enumerate() {
                                println!("{}", console::style(format!("    {}: {}", i + 1, sense)).green());
                            }
                        }
                    };

                    // Handle win / fail
                    match game_status {
                        GameStatus::Won(round) => {
                            stats.win(args.state.is_some(), guesses);
                            break if is_tty {
                                println!(
                                    "{}",
                                    console::style(format!("You won in {round} guesses!"))
                                        .bold()
                                        .magenta()
                                );

                                print_definition(&guesses.last().unwrap().0);
                            } else {
                                println!("CORRECT {round}");
                            };
                        }
                        GameStatus::Failed(answer) => {
                            stats.fail(args.state.is_some(), guesses, &answer);
                            break if is_tty {
                                println!(
                                    "{}",
                                    console::style(format!("You lose! The answer is: {}", answer))
                                        .bold()
                                        .red()
                                );

                                print_definition(&answer);
                            } else {
                                println!("FAILED {}", answer);
                            };
                        }
                        GameStatus::Going => (),
                    }
                }
                Err(error) => print_error(is_tty, &error),
            }
        }

        // Print statistics
        if args.stats {
            stats.print(is_tty);
        }

        // Ask whether to start a new game
        if is_tty && args.word.is_none() {
            loop {
                print!(
                    "Would you like to start a new game? {} ",
                    console::style("[Y/N]").bold().blue()
                );
                flush();
                match read_line() {
                    None => exit_game(is_tty),
                    Some(line) => match line.as_str() {
                        "Y" | "y" => break println!(""),
                        "N" | "n" => exit_game(is_tty),
                        _ => continue,
                    },
                }
            }
        } else if !is_tty {
            match read_line() {
                None => exit_game(is_tty),
                Some(line) => {
                    if line == "Y" {
                        // Continue game loop
                        continue;
                    } else {
                        exit_game(is_tty);
                    }
                }
            }
        } else {
            exit_game(is_tty);
        }
    }
}

// For compiling into Wasm
#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::start_web(
        "canvas",
        eframe::WebOptions::default(),
        Box::new(|cc| Box::new(WordleApp::new(cc))),
    )
    .unwrap();
}
