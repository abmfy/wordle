use egui::RichText;

use crate::stats::Stats;

use super::{colors, metrics};

/// Statistics panel
pub fn stats(ui: &mut egui::Ui, dark: bool, stats: &Stats) {
    ui.collapsing("Statistics", |ui| {
        ui.set_min_width(metrics::PANEL_WIDTH);

        ui.label(
            RichText::new(format!("Wins:  {}", stats.get_wins()))
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
            RichText::new(format!("Average tries: {:.2}", stats.get_average_tries())).strong(),
        );

        ui.label("");

        ui.label(RichText::new(format!("Favorite words:")).strong());
        for (word, times) in stats.get_favorite_words() {
            ui.label(format!("{word}: used {times} times"));
        }
    });
}
