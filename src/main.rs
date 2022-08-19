use clap::{Parser, ArgGroup};
use console;
use rand::Rng;
use std::{io::{self, Write}, collections::HashSet};

mod builtin_words;
mod game;

use game::{Game, LetterStatus, GuessStatus, GameStatus};

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author="abmfy", about="A Wordle game, refined")]
#[clap(group(ArgGroup::new("answer").args(&["word", "random"])))]
struct Args {
    /// Specify the answer
    #[clap(short, long, value_parser=is_in_answer_list)]
    word: Option<String>,

    /// Randomly choose the answer
    #[clap(short, long)]
    random: bool,

    /// Enter difficult mode, where you must guess according to the former result
    #[clap(short='D', long)]
    difficult: bool
}

fn is_in_answer_list(word: &str) -> Result<String, String> {
    if builtin_words::FINAL.binary_search(&word.to_lowercase().as_ref()).is_ok() {
        Ok(word.to_string())
    } else {
        Err(game::Error::BadAnswer.what())
    }
}

/// Read a line, trimmed. Return None if EOF encountered
fn read_line() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line.trim().to_string())
    }
}

/// Flush the output
fn flush() {
    io::stdout().flush().unwrap();
}

/// Print an error message
fn print_error(is_tty: bool, error: &game::Error) {
    println!("{}", if is_tty {error.what()} else {"INVALID".to_string()})
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
                print!("{}", guesses[i].1[j].colored_char(c.to_uppercase().nth(0).unwrap()));
            }
            println!("");
        } else {
            println!("{}", console::style("_____").dim());
        }
    }
}

// Print the alphabet, in tty mode
fn print_alphabet(alphabet: &[LetterStatus]) {
    const ROW1: &str = "qwertyuiop";
    const ROW2: &str = "asdfghjkl";
    const ROW3: &str = "zxcvbnm";
    for row in [ROW1, ROW2, ROW3] {
        for c in row.chars() {
            print!("{}", alphabet[game::get_index(c)].colored_char(c.to_uppercase().nth(0).unwrap()));
        }
        println!("");
    }    
}

/// Exit game and provided a message if in tty mode
fn exit_game(is_tty: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_tty {
        println!("Goodbye!");
    }
    Ok(())
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let is_tty = atty::is(atty::Stream::Stdout);
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

        print!("{}", console::style("Your name: ").bold().red());
        flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome, {}!", line.trim());
    }

    // Avoid word recurrence when in random mode
    let mut used_answers = HashSet::<usize>::new();

    // Game loop
    loop {
        // Did not provide answer
        let mut game = if args.word.is_none() {
            // Random mode
            if args.random {
                let index = loop {
                    let now = rand::thread_rng().gen_range(0..builtin_words::FINAL.len());
                    if !used_answers.contains(&now) {
                        used_answers.insert(now);
                        break now
                    } else {
                        continue
                    }
                };

                Game::new(builtin_words::FINAL[index], args.difficult).unwrap()
            } else {
                if is_tty {
                    println!("Please choose an answer for the game.");
                }
                loop {
                    let answer: String = match read_line() {
                        Some(word) => word,
                        None => return exit_game(is_tty)
                    };
                    let answer = answer.to_lowercase();
                    match Game::new(&answer, args.difficult) {
                        Ok(game) => break game,
                        Err(error) => print_error(is_tty, &error)
                    }
                }
            }
            
        } else {
            Game::new(args.word.as_ref().unwrap(), args.difficult).unwrap()
        };

        loop {
            if is_tty {
                print!("{}", console::style(format!("Guess {}: ", game.get_round() + 1)).blue());
                flush();
            }

            let word: String = match read_line() {
                Some(word) => word,
                None => return exit_game(is_tty)
            };
            let word = word.to_lowercase();
            let result = game.guess(&word);
            match result {
                Ok((game_status, guesses, alphabet)) => {
                    if is_tty {
                        print_guess_history(guesses);
                        println!("--------------");
                        print_alphabet(alphabet);
                        match game_status {
                            GameStatus::Won(round) => break println!("You won in {round} guesses!"),
                            GameStatus::Failed(answer) => break println!("You lose! The answer is: {}", answer.to_uppercase()),
                            GameStatus::Going => ()
                        }
                    } else {
                        print_status(&guesses.last().unwrap().1);
                        print!(" ");
                        print_status(alphabet);
                        println!("");
                        match game_status {
                            GameStatus::Won(round) => break println!("CORRECT {round}"),
                            GameStatus::Failed(answer) => break println!("FAILED {}", answer.to_uppercase()),
                            GameStatus::Going => ()
                        }
                    }
                },
                Err(error) => print_error(is_tty, &error)
            }
        }
        // Ask whether to start a new game
        if is_tty && args.word.is_none() {
            loop {
                print!("Would you like to start a new game? {} ", console::style("[Y/N]").bold());
                flush();
                match read_line() {
                    None => return Ok(()),
                    Some(line) => {
                        match line.as_str() {
                            "Y" | "y" => break,
                            "N" | "n" => return exit_game(is_tty),
                            _ => continue
                        }
                    }
                }
            }

        } else {
            return exit_game(is_tty);
        }
    }
}
