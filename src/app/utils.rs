use egui::{Color32, Id};

use super::keyboard;

/// Get the width of the screen
pub fn get_screen_width(ui: &egui::Ui) -> f32 {
    ui.input().screen_rect.width()
}

/// Get the height of the screen
pub fn get_screen_height(ui: &egui::Ui) -> f32 {
    ui.input().screen_rect.height()
}

/// Check if the device is a phone
pub fn is_phone(ui: &egui::Ui) -> bool {
    return keyboard::get_keyboard_size_factor(ui) < 1.0;
}

// Make a smooth transition when color changes
pub fn animate_color(ctx: &egui::Context, id: String, color: Color32) -> Color32 {
    const DURATION: f32 = 0.5;

    fn animate_value(ctx: &egui::Context, id: &String, suffix: &str, value: u8) -> u8 {
        ctx.animate_value_with_time(Id::new(id.clone() + suffix), value as f32, DURATION) as u8
    }

    let r = animate_value(ctx, &id, "r", color.r());
    let g = animate_value(ctx, &id, "g", color.g());
    let b = animate_value(ctx, &id, "b", color.b());

    Color32::from_rgb(r, g, b)
}
