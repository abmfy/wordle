use egui::{CollapsingHeader, FontData, FontDefinitions, FontFamily, Frame, RichText};
use rand::seq::SliceRandom;
use rand::SeedableRng;

mod colors;
mod keyboard;
mod letter;
mod metrics;
mod utils;

use crate::args::{self, Args};
use crate::builtin_words;
use crate::game::{Game, GameStatus, LetterStatus};
use crate::stats::State;

use letter::Letter;

use keyboard::keyboard;
use letter::letter;

/// App state persistence
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WordleApp {
    args: Args,
    state: State,
    game: Option<Game>,
    game_status: Option<GameStatus>,
    guess: String,
    #[serde(skip)]
    word_list: Vec<String>,
    #[serde(skip)]
    answer_list: Vec<String>,
}

impl Default for WordleApp {
    fn default() -> Self {
        Self {
            args: Args::default(),
            state: State::default(),
            game: None,
            game_status: None,
            guess: "".to_string(),
            word_list: vec![],
            answer_list: vec![],
        }
    }
}

impl WordleApp {
    /// App initialization
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        // Load fonts
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "NY".to_string(),
            FontData::from_static(include_bytes!("../assets/NewYorkExtraLarge-Bold.otf")),
        );

        fonts
            .families
            .insert(FontFamily::Name("NY".into()), vec!["NY".to_string()]);

        fonts.font_data.insert(
            "SF".to_string(),
            FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Bold.otf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "SF".to_string());

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state
        let mut app: WordleApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // Load word lists
        app.word_list = builtin_words::ACCEPTABLE
            .iter()
            .map(|s| s.to_string())
            .collect();
        app.answer_list = builtin_words::FINAL.iter().map(|s| s.to_string()).collect();

        let mut rng =
            rand::rngs::StdRng::seed_from_u64(app.args.seed.unwrap_or(args::DEFAULT_SEED));

        app.answer_list.shuffle(&mut rng);

        app
    }
}

impl eframe::App for WordleApp {
    /// Save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Update UI and handle input events
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The header
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new("Wordle")
                        .family(FontFamily::Name("NY".into()))
                        .size(72.0),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Setting panel
            Frame::window(ui.style()).show(ui, |ui| {
                CollapsingHeader::new("Settings").show(ui, |ui| {
                    // TODO: unimplemented
                    if ui.button("Restart").clicked() {
                        self.game = None;
                        self.guess.clear();
                    }
                });
            });

            // Statistics panel
            Frame::window(ui.style()).show(ui, |ui| {
                CollapsingHeader::new("Statistics").show(ui, |ui| {
                    // TODO: unimplemented
                    ui.label("Ho");
                });
            });

            // Start a new game
            if self.game.is_none() {
                let mut day = self.args.day.unwrap_or(0);

                self.game = Game::new(
                    &self.answer_list[day as usize],
                    self.args.difficult,
                    &self.answer_list,
                )
                .ok();

                // Yet another day of playing wordle...
                // The mod is here to avoid overflow
                day += 1;
                day %= self.answer_list.len() as u32;
                self.args.day = Some(day);

                self.game_status = Some(GameStatus::Going);
            }

            // We are in a game now
            let game = self.game.as_mut().unwrap();

            // The letter grid
            for i in 0..metrics::ROWS as usize {
                for j in 0..metrics::COLUMNS as usize {
                    // Already guessed
                    if i < game.get_round() {
                        let guess = &game.get_guesses()[i];
                        let letter_char =
                            Some(guess.0.chars().nth(j).unwrap().to_ascii_uppercase());
                        let status = guess.1[j];
                        letter(
                            ui,
                            i as i32,
                            j as i32,
                            &Letter {
                                letter: letter_char,
                                status,
                            },
                        );
                    } else if i == game.get_round() && j < self.guess.len() {
                        // We'll input words in this row, and the jth letter already input
                        letter(
                            ui,
                            i as i32,
                            j as i32,
                            &Letter {
                                letter: self.guess.chars().nth(j),
                                status: LetterStatus::Unknown,
                            },
                        )
                    } else {
                        // Blank letter
                        letter(
                            ui,
                            i as i32,
                            j as i32,
                            &Letter {
                                letter: None,
                                status: LetterStatus::Unknown,
                            },
                        );
                    }
                }
            }

            // Render the keyboard and get keyboard input
            if let Some(key) = keyboard(ui, game.get_alphabet()) {
                if self.game_status == Some(GameStatus::Going) {
                    match key {
                        // Guess
                        keyboard::ENTER => {
                            let result = game.guess(&self.guess.to_lowercase(), &self.word_list);
                            match result {
                                Ok(game_status) => {
                                    // Clear guess for next guess to use
                                    self.guess.clear();
                                    match game_status {
                                        GameStatus::Won(round) => {
                                            println!("Won! {round}");
                                        }
                                        GameStatus::Failed(ref answer) => {
                                            println!("Failed {answer}");
                                        }
                                        GameStatus::Going => (),
                                    }

                                    // Save game status
                                    self.game_status = Some(game_status);
                                }
                                Err(error) => println!("{}", error.what()),
                            }
                        }
                        keyboard::BACKSPACE => {
                            self.guess.pop();
                        }
                        // Enter a letter
                        _ => {
                            // Avoid entering more than 5 letters
                            if self.guess.len() < metrics::COLUMNS as usize {
                                self.guess.push(key);
                            }
                        }
                    }
                }
            }
        });
    }
}
