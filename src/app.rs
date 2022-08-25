use egui::{FontData, FontDefinitions, FontFamily, Frame, Pos2, RichText, Window};
use rand::seq::SliceRandom;
use rand::SeedableRng;

mod colors;
mod definition;
mod grid;
mod keyboard;
mod letter;
mod metrics;
mod settings;
mod stats;
mod utils;
mod visuals;

use crate::args::{self, Args};
use crate::builtin_words;
use crate::game::{Game, GameStatus};
use crate::stats::Stats;

use definition::definition;
use grid::grid;
use keyboard::keyboard;
use settings::settings;
use stats::stats;

/// App state persistence
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WordleApp {
    args: Args,
    stats: Stats,
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
            stats: Stats::default(),
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
            "SFM".to_string(),
            FontData::from_static(include_bytes!("../assets/SF-Mono-Medium.otf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "SFM".to_string());

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state
        let mut app: WordleApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // Load default args
        if app.args.seed.is_none() {
            app.args.seed = Some(args::DEFAULT_SEED);
        }
        if app.args.day.is_none() {
            app.args.day = Some(args::DEFAULT_DAY - 1);
        }

        // Load word lists
        app.word_list = builtin_words::ACCEPTABLE
            .iter()
            .map(|s| s.to_uppercase())
            .collect();
        app.answer_list = builtin_words::FINAL
            .iter()
            .map(|s| s.to_uppercase())
            .collect();

        app.shuffle_answer_list(app.args.seed.unwrap());

        // Start a new game when first run
        if app.game.is_none() {
            app.start();
        }

        app
    }

    /// Shuffle the answer list
    fn shuffle_answer_list(&mut self, seed: u64) {
        // Sort the answer list first to produce reproducible results
        self.answer_list.sort();

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        self.answer_list.shuffle(&mut rng);
    }

    /// Start a new game at specified day
    fn start(&mut self) {
        let day = self.args.day.unwrap();

        self.game = Game::new(
            &self.answer_list[day as usize],
            self.args.difficult,
            &self.answer_list,
        )
        .ok();

        self.game_status = Some(GameStatus::Going);

        self.guess.clear();
    }

    /// Increase the day count
    fn another_day(&mut self) {
        let mut day = self.args.day.unwrap();
        // Yet another day of playing wordle...
        // The mod is here to avoid overflow
        day += 1;
        day %= self.answer_list.len() as u32;
        self.args.day = Some(day);
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
            // Use a window to contain the panels on a phone
            if utils::is_phone(ui) {
                Window::new(RichText::new("Explore").size(metrics::PANEL_TITLE_FONT_SIZE))
                    .auto_sized()
                    .fixed_pos(Pos2 {
                        x: metrics::PADDING,
                        y: metrics::HEADER_HEIGHT + metrics::PANEL_MARGIN,
                    })
                    .show(ui.ctx(), |ui| {
                        // Setting panel
                        settings(ui, self);

                        // Stats panel
                        stats(ui, self.args.difficult, &self.stats);

                        // Definition panel
                        definition(ui, self);
                    });
            } else {
                // Setting panel
                Frame::window(ui.style()).show(ui, |ui| {
                    settings(ui, self);
                });

                // Stats panel
                Frame::window(ui.style()).show(ui, |ui| {
                    stats(ui, self.args.difficult, &self.stats);
                });

                // Definition panel
                Frame::window(ui.style()).show(ui, |ui| {
                    definition(ui, self);
                });
            }

            // We are in a game now
            let game = self.game.as_mut().unwrap();

            // The letter grid
            grid(ui, game, &mut self.guess, self.args.difficult);

            // Render the keyboard and get keyboard input
            if let Some(key) = keyboard(
                ui,
                self.args.difficult,
                game.get_alphabet(),
                self.game_status.as_ref().unwrap(),
                game.validate_guess(self.args.difficult, false, &self.guess, &self.word_list)
                    .is_ok(),
            ) {
                if self.game_status == Some(GameStatus::Going) {
                    match key {
                        // Guess
                        keyboard::ENTER => {
                            let result = game.guess(&self.guess, &self.word_list);
                            match result {
                                Ok(game_status) => {
                                    // Update stats
                                    self.stats.update_guess(&self.guess);
                                    match &game_status {
                                        GameStatus::Won(round) => {
                                            self.stats.win_with_guesses_updated(*round)
                                        }
                                        GameStatus::Failed(_) => {
                                            self.stats.fail_with_guesses_updated()
                                        }
                                        GameStatus::Going => (),
                                    }

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
                    // When game overed use either ENTER or BACKSPACE to start a new game
                    if key == keyboard::BACKSPACE || key == keyboard::ENTER {
                        self.another_day();
                        self.start();
                    }
                }
            }
        });
    }
}
