use egui::ScrollArea;

use crate::dict::DICT;
use crate::game::GameStatus;

use super::{metrics, WordleApp};

/// Definition panel
pub fn definition(ui: &mut egui::Ui, app: &WordleApp) {
    ui.collapsing("Definition", |ui| {
        ui.set_max_width(metrics::PANEL_WIDTH);
        ui.set_max_height(metrics::PANEL_HEIGHT);
        let answer = match app.game_status.as_ref().unwrap() {
            // Show help message when the game is going
            GameStatus::Going => {
                ui.label("Come back later when the game is over!");
                ui.label("If you need some help, try out typing 'HINT'!");
                return;
            }
            GameStatus::Won(_) => &app.game.as_ref().unwrap().get_guesses().last().unwrap().0,
            GameStatus::Failed(ref answer) => answer,
        };

        // Show the definition
        ScrollArea::vertical().show(ui, |ui| {
            // ui.set_max_height(50.0);
            for (i, sense) in DICT.get(answer).unwrap().iter().enumerate() {
                ui.label(format!("{}: {sense}", i + 1));
            }
        });
    });
}
