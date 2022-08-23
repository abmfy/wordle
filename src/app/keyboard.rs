use egui::{Align2, Color32, CursorIcon, FontFamily, Key, Modifiers, Pos2, Rect, Sense, Vec2};

use crate::game::{self, Alphabet, LetterStatus};

use super::{
    colors, metrics,
    utils::{self, animate_color},
};

pub const ENTER: char = '\n';
pub const BACKSPACE: char = '\x08';

pub fn get_key_fill_color(status: &LetterStatus) -> Color32 {
    match status {
        LetterStatus::Unknown => colors::GRAY,
        LetterStatus::Red => colors::DARK_GRAY,
        LetterStatus::Yellow => colors::YELLOW,
        LetterStatus::Green => colors::GREEN,
    }
}

pub fn get_key_text_color(status: &LetterStatus) -> Color32 {
    match status {
        LetterStatus::Unknown => colors::BLACK,
        _ => colors::WHITE,
    }
}

/// A key in the keyboard widget, returns whether this key is clicked
pub fn key(ui: &mut egui::Ui, c: char, status: &LetterStatus, x: f32, y: f32) -> bool {
    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: match c {
                ENTER | BACKSPACE => metrics::KEY_WIDTH_LARGE,
                _ => metrics::KEY_WIDTH,
            },
            y: metrics::KEY_HEIGHT,
        },
    );

    // We need to sense click event
    let mut response = ui.allocate_rect(rect, Sense::click());

    // Render different colors depending on cursor state
    let color = {
        let mut color = get_key_fill_color(status);
        color = animate_color(ui, format!("key{c}f"), color);
        if response.hovered() {
            color = color.linear_multiply(0.8);
        }
        if response.is_pointer_button_down_on() {
            color = color.linear_multiply(1.5);
        }
        color
    };

    // Change cursor style when hovered
    response = response.on_hover_cursor(CursorIcon::PointingHand);

    // Paint rect
    ui.painter()
        .rect(rect, metrics::KEY_RADIUS, color, (0.0, colors::GRAY));
    // Paint text
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        match c {
            ENTER => "ENTER".to_string(),
            BACKSPACE => "⌫".to_string(),
            _ => c.to_string(),
        },
        egui::FontId {
            size: match c {
                BACKSPACE => metrics::KEY_FONT_SIZE * 1.5,
                _ => metrics::KEY_FONT_SIZE,
            },
            family: FontFamily::Proportional,
        },
        animate_color(ui, format!("key{c}t"), get_key_text_color(status)),
    );

    response.clicked()
}

/// The keyboard widget
/// Returns which key is pressed
pub fn keyboard(ui: &mut egui::Ui, alphabet: &Alphabet) -> Option<char> {
    let mut pressed: Option<char> = None;

    // Render the r-th row
    fn row(
        r: i32,
        ui: &mut egui::Ui,
        pressed: &mut Option<char>,
        alphabet: &Alphabet,
        letters: &str,
    ) {
        let mut row_width = letters.len() as f32 * (metrics::KEY_WIDTH + metrics::KEY_H_MARGIN)
            - metrics::KEY_H_MARGIN;
        if letters.contains(ENTER) {
            row_width += metrics::KEY_WIDTH_LARGE - metrics::KEY_WIDTH;
        }
        if letters.contains(BACKSPACE) {
            row_width += metrics::KEY_WIDTH_LARGE - metrics::KEY_WIDTH;
        }
        let mut allocated_width = 0.0f32;
        for c in letters.chars() {
            let x = {
                let x = utils::get_screen_width(ui) / 2.0 - row_width / 2.0 + allocated_width;
                allocated_width += metrics::KEY_H_MARGIN
                    + if c == ENTER {
                        metrics::KEY_WIDTH_LARGE
                    } else {
                        metrics::KEY_WIDTH
                    };
                x
            };
            let y = utils::get_screen_height(ui)
                - (metrics::KEY_HEIGHT + metrics::KEY_V_MARGIN) * (3.0 + 1.0 - r as f32);
            // Detect keystroke
            match c {
                ENTER | BACKSPACE => {
                    if key(ui, c, &LetterStatus::Unknown, x, y) {
                        *pressed = Some(c)
                    }
                }
                _ => {
                    if key(
                        ui,
                        c,
                        &alphabet[game::get_index(c.to_ascii_lowercase())],
                        x,
                        y,
                    ) {
                        *pressed = Some(c);
                    }
                }
            }
        }
    }
    row(1, ui, &mut pressed, &alphabet, "QWERTYUIOP");
    row(2, ui, &mut pressed, &alphabet, "ASDFGHJKL");
    row(
        3,
        ui,
        &mut pressed,
        &alphabet,
        &format!("{ENTER}ZXCVBNM{BACKSPACE}"),
    );

    // Track physical keyboard input
    const TRACKED_KEYS: [egui::Key; 28] = [
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
        Key::Enter,
        Key::Backspace,
    ];

    for key in TRACKED_KEYS {
        // Early return to avoid key conflict
        if ui.input_mut().consume_key(Modifiers::NONE, key) {
            pressed = match key {
                Key::Enter => Some(ENTER),
                Key::Backspace => Some(BACKSPACE),
                _ => format!("{key:?}").chars().nth(0),
            }
        }
        // Shift is allowed (though not required)
        if ui.input_mut().consume_key(Modifiers::SHIFT, key) {
            pressed = match key {
                Key::Enter => Some(ENTER),
                Key::Backspace => Some(BACKSPACE),
                _ => format!("{key:?}").chars().nth(0),
            }
        }
    }

    pressed
}
