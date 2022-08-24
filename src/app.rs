use egui::{CollapsingHeader, FontData, FontDefinitions, FontFamily, Frame, Label, RichText};
use rand::seq::SliceRandom;
use rand::SeedableRng;

mod colors;
mod keyboard;
mod letter;
mod metrics;
mod utils;
mod visuals;

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
            .insert(FontFamily::Name("SF".into()), vec!["SF".to_string()]);

        fonts.font_data.insert(
            "SF_R".to_string(),
            FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Regular.otf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "SF_R".to_string());

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
            .map(|s| s.to_uppercase())
            .collect();
        app.answer_list = builtin_words::FINAL
            .iter()
            .map(|s| s.to_uppercase())
            .collect();

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
        // Enable dark visuals in hard mode
        ctx.set_visuals(visuals::get_visuals(ctx, self.args.difficult));

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
                    ui.set_max_width(200.0);
                    // TODO: unimplemented
                    // ui.add(Label::new("The settings will go into effect next game.").wrap(true));
                    // Hard mode
                    if ui.checkbox(&mut self.args.difficult, "Hard Mode").changed() {
                        if let Some(ref mut game) = self.game {
                            game.set_difficult(self.args.difficult);
                        }
                    }
                    ui.add(
                        Label::new("Any revealed hints must be used in subsequent guesses.")
                            .wrap(true),
                    );
                });
            });

            // Statistics panel
            Frame::window(ui.style()).show(ui, |ui| {
                CollapsingHeader::new("Statistics").show(ui, |ui| {
                    // TODO: unimplemented
                    ui.set_max_width(200.0);
                    ui.label("How about we explore the area ahead of us later?");
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
                            self.args.difficult,
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
                            self.args.difficult,
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
                            self.args.difficult,
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
            if let Some(key) = keyboard(
                ui,
                self.args.difficult,
                game.get_alphabet(),
                self.game_status.as_ref().unwrap(),
                game.is_valid_guess(&self.guess, &self.word_list),
            ) {
                if self.game_status == Some(GameStatus::Going) {
                    match key {
                        // Guess
                        keyboard::ENTER => {
                            let result = game.guess(&self.guess, &self.word_list);
                            match result {
                                Ok(game_status) => {
                                    // Clear guess for next guess to use
                                    self.guess.clear();

                                    // Save game status
                                    self.game_status = Some(game_status);
                                }
                                // Do nothing because we've indicated the guess is invalid by the enter button
                                Err(_) => (),
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
                } else {
                    // When game overed use either ENTER or BACKSPACE to restart
                    if key == keyboard::BACKSPACE || key == keyboard::ENTER {
                        self.game = None;
                    }
                }
            }
        });
    }
}
