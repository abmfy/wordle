use clap::{Parser};
use console;
use std::{io::{self, Write}, process::exit};

mod builtin_words;
mod game;

use game::{Game, LetterStatus, GuessStatus, GameStatus};

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author="abmfy", about="A Wordle game, refined")]
struct Args {
    #[clap(short, long, value_parser=is_in_answer_list)]
    answer: Option<String>
}

fn is_in_answer_list(word: &str) -> Result<String, String> {
    if builtin_words::FINAL.contains(&word.to_lowercase().as_ref()) {
        Ok(word.to_string())
    } else {
        Err(game::Error::BadAnswer.what())
    }
}

/// Read a line, trimmed
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
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

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // println!("{args:?}");

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

    let mut game = if args.answer.is_none() {
        println!("Please choose an answer for the game.");
        loop {
            let answer: String = read_line();
            let answer = answer.to_lowercase();
            match Game::new(&answer) {
                Ok(game) => break game,
                Err(error) => print_error(is_tty, &error)
            }
        }
    } else {
        Game::new(args.answer.unwrap().as_ref()).unwrap()
    };

    loop {
        if is_tty {
            print!("{}", console::style(format!("Guess {}: ", game.get_round() + 1)).blue());
            flush();
        }

        let word: String = read_line();
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

    Ok(())
}
