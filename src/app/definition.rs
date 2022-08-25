use egui::ScrollArea;
use egui::{CollapsingHeader, Frame};

use crate::dict::DICT;
use crate::game::GameStatus;

use super::WordleApp;

/// Definition panel
pub fn definition(ui: &mut egui::Ui, app: &WordleApp) {
    Frame::window(ui.style()).show(ui, |ui| {
        CollapsingHeader::new("Definition").show(ui, |ui| {
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                ui.set_max_width(200.0);

                let answer = match app.game_status.as_ref().unwrap() {
                    // Show help message when the game is going
                    GameStatus::Going => {
                        ui.label("Come back later when the game is over!");
                        ui.label("If you need some help, try out typing 'HINT'!");
                        return;
                    }
                    GameStatus::Won(_) => {
                        &app.game.as_ref().unwrap().get_guesses().last().unwrap().0
                    }
                    GameStatus::Failed(ref answer) => answer,
                };

                // Show the definition
                for (i, sense) in DICT.get(answer).unwrap().iter().enumerate() {
                    ui.label(format!("{}: {sense}", i + 1));
                }
            });
        });
    });
}
