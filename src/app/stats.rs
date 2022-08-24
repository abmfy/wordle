use egui::{CollapsingHeader, Frame, RichText};

use crate::stats::Stats;

use super::colors;

/// Statistics panel
pub fn stats(ui: &mut egui::Ui, dark: bool, stats: &Stats) {
    Frame::window(ui.style()).show(ui, |ui| {
        CollapsingHeader::new("Statistics").show(ui, |ui| {
            ui.set_max_width(200.0);

            ui.label(
                RichText::new(format!("Wins: {}", stats.get_wins()))
                    .strong()
                    .color(if dark {
                        colors::GREEN
                    } else {
                        colors::DARK_MODE_GREEN
                    }),
            );
            ui.label(
                RichText::new(format!("Fails: {}", stats.get_fails()))
                    .strong()
                    .color(if dark {
                        colors::YELLOW
                    } else {
                        colors::DARK_MODE_YELLOW
                    }),
            );
            ui.label(
                RichText::new(format!("Average tries: {}", stats.get_average_tries())).strong(),
            );

            ui.label("");

            ui.label(RichText::new(format!("Favotite words:")).strong());
            for (word, times) in stats.get_favorite_words() {
                ui.label(format!("{word}: used {times} times"));
            }
        });
    });
}
