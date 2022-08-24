use egui::{CollapsingHeader, DragValue, Frame, Label};

use super::WordleApp;

/// Settings panel
pub fn settings(ui: &mut egui::Ui, app: &mut WordleApp) {
    Frame::window(ui.style()).show(ui, |ui| {
        CollapsingHeader::new("Settings  ").show(ui, |ui| {
            let game = app.game.as_mut().unwrap();

            ui.set_max_width(200.0);

            // Hard mode
            if ui.checkbox(&mut app.args.difficult, "Hard Mode").changed() {
                game.set_difficult(app.args.difficult);
            }
            ui.add(Label::new("Any revealed hints must be used in subsequent guesses.").wrap(true));

            // Random seed
            ui.horizontal(|ui| {
                ui.label("Seed: ");

                let seed_before = app.args.seed.unwrap();

                ui.add(DragValue::new(app.args.seed.as_mut().unwrap()));

                let seed_after = app.args.seed.unwrap();

                // If seed is changed, shuffle the answer list again
                if seed_after != seed_before {
                    app.shuffle_answer_list(seed_after);
                }
            });

            // Day
            ui.horizontal(|ui| {
                ui.label("Day:  ");
                ui.add(
                    DragValue::new(app.args.day.as_mut().unwrap())
                        .clamp_range(0..=app.answer_list.len() - 1)
                        .custom_formatter(|n, _| {
                            // Plus one when shown to the user cuz humans count from one. Phew.
                            format!("{}", n + 1.0)
                        }),
                );
            });

            ui.add(
                Label::new("The above two settings won't go into effect until next game.")
                    .wrap(true),
            );

            if ui.button("Restart").clicked() {
                app.start();
            };
        });
    });
}
