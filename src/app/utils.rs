use egui::{Color32, Id};

/// Get the width of the screen
pub fn get_screen_width(ui: &egui::Ui) -> f32 {
    ui.input().screen_rect.width()
}

/// Get the height of the screen
pub fn get_screen_height(ui: &egui::Ui) -> f32 {
    ui.input().screen_rect.height()
}

// Make a smooth transition when color changes
pub fn animate_color(ui: &egui::Ui, id: String, color: Color32) -> Color32 {
    const DURATION: f32 = 1.0;

    fn animate_value(ui: &egui::Ui, id: &String, suffix: &str, value: u8) -> u8 {
        ui.ctx()
            .animate_value_with_time(Id::new(id.clone() + suffix), value as f32, DURATION)
            as u8
    }

    let r = animate_value(ui, &id, "r", color.r());
    let g = animate_value(ui, &id, "g", color.g());
    let b = animate_value(ui, &id, "b", color.b());

    Color32::from_rgb(r, g, b)
}
