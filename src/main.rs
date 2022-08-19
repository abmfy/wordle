use console;
use std::io::{self, Write};

mod builtin_words;
mod game;

use game::{Game, LetterStatus, GameStatus};

/// Read a line, trimmed
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

/// Print an error message
fn print_error(is_tty: bool, error: &game::Error) {
    println!("{}", if is_tty {error.what()} else {"INVALID".to_string()})
}

/// Print status of letters
fn print_status(is_tty: bool, status: &[LetterStatus]) {
    if is_tty {
        status.iter().for_each(|s| {
            print!("{}", s.colored_char());
        });
    } else {
        print!("{}", String::from_iter(status.iter().map(|s| s.to_char())));
    }
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to Wordle, {}!", line.trim());
    }

    // example: print arguments
    // print!("Command line arguments: ");
    // for arg in std::env::args() {
    //     print!("{} ", arg);
    // }
    // println!("");
    // TODO: parse the arguments in `args`

    let mut game = loop {
        let answer: String = read_line();
        let answer = answer.to_lowercase();
        match Game::new(&answer) {
            Ok(game) => break game,
            Err(error) => print_error(is_tty, &error)
        }
    };

    loop {
        let word: String = read_line();
        let word = word.to_lowercase();
        match game.guess(&word) {
            Ok((game_status, guess_status, alphabet)) => {
                if false && is_tty {
                    unimplemented!();
                } else {
                    print_status(is_tty, &guess_status);
                    print!(" ");
                    print_status(is_tty, alphabet);
                    println!("");
                }
                match game_status {
                    GameStatus::Won(round) => break println!("CORRECT {round}"),
                    GameStatus::Failed(answer) => break println!("FAILED {}", answer.to_uppercase()),
                    GameStatus::Going => ()
                }
            },
            Err(error) => print_error(is_tty, &error)
        }
    }

    Ok(())
}
