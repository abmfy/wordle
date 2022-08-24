use egui::{Align2, Color32, CursorIcon, FontFamily, Key, Modifiers, Pos2, Rect, Sense, Vec2};

use crate::game::{self, Alphabet, GameStatus, LetterStatus};

use super::{colors, metrics, utils};

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
fn letter_key(ui: &mut egui::Ui, c: char, status: &LetterStatus, x: f32, y: f32) -> bool {
    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH,
            y: metrics::KEY_HEIGHT,
        },
    );

    // We need to sense click event
    let mut response = ui.allocate_rect(rect, Sense::click());

    // Render different colors depending on cursor state
    let color = {
        let mut color = get_key_fill_color(status);
        color = utils::animate_color(ui, format!("key{c}f"), color);
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
        c,
        egui::FontId {
            size: metrics::KEY_FONT_SIZE,
            family: FontFamily::Proportional,
        },
        utils::animate_color(ui, format!("key{c}t"), get_key_text_color(status)),
    );

    response.clicked()
}

/// Enter key, meanwhile shows a message when game over
fn enter_key(ui: &mut egui::Ui, status: &GameStatus, valid: bool, x: f32, y: f32) -> bool {
    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH_LARGE,
            y: metrics::KEY_HEIGHT,
        },
    );

    // We need to sense click event when the input is valid
    let mut response = ui.allocate_rect(
        rect,
        if valid {
            Sense::click()
        } else {
            Sense::hover()
        },
    );

    // Render different colors depending on cursor state
    let color = {
        let mut color = match status {
            GameStatus::Going => {
                if valid {
                    colors::GRAY
                } else {
                    colors::DARK_GRAY
                }
            }
            GameStatus::Won(_) => colors::GREEN,
            GameStatus::Failed(_) => colors::YELLOW,
        };
        color = utils::animate_color(ui, format!("key{}f", "ENTER"), color);
        if valid {
            if response.hovered() {
                color = color.linear_multiply(0.8);
            }
            if response.is_pointer_button_down_on() {
                color = color.linear_multiply(1.5);
            }
        }
        color
    };

    // Change cursor style when hovered
    response = response.on_hover_cursor(match status {
        GameStatus::Going => {
            if valid {
                CursorIcon::PointingHand
            } else {
                CursorIcon::NotAllowed
            }
        }
        GameStatus::Won(_) | GameStatus::Failed(_) => CursorIcon::Default,
    });

    // Paint rect
    ui.painter()
        .rect(rect, metrics::KEY_RADIUS, color, (0.0, colors::GRAY));
    // Paint text
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        match status {
            GameStatus::Going => "ENTER".to_string(),
            GameStatus::Won(_) => "Bravo!".to_string(),
            GameStatus::Failed(answer) => answer.to_uppercase(),
        },
        egui::FontId {
            size: metrics::KEY_FONT_SIZE,
            family: FontFamily::Proportional,
        },
        utils::animate_color(
            ui,
            format!("key{}t", "ENTER"),
            match status {
                GameStatus::Going => {
                    if valid {
                        colors::BLACK
                    } else {
                        colors::GRAY
                    }
                }
                GameStatus::Won(_) | GameStatus::Failed(_) => colors::WHITE,
            },
        ),
    );

    response.clicked()
}

/// Backspace key, meanwhile shows a message when game over
fn backspace_key(ui: &mut egui::Ui, status: &GameStatus, x: f32, y: f32) -> bool {
    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH_LARGE,
            y: metrics::KEY_HEIGHT,
        },
    );

    // We need to sense click event
    let mut response = ui.allocate_rect(rect, Sense::click());

    // Render different colors depending on cursor state
    let color = {
        let mut color = match status {
            GameStatus::Going => colors::GRAY,
            GameStatus::Won(_) => colors::GREEN,
            GameStatus::Failed(_) => colors::YELLOW,
        };
        color = utils::animate_color(ui, format!("key{}f", "BACKSPACE"), color);
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
        match status {
            GameStatus::Going => "⌫",
            GameStatus::Won(_) | GameStatus::Failed(_) => "RESTART",
        },
        egui::FontId {
            size: metrics::KEY_FONT_SIZE
                * if status == &GameStatus::Going {
                    1.5
                } else {
                    0.8
                },
            family: FontFamily::Proportional,
        },
        utils::animate_color(
            ui,
            format!("key{}t", "BACKSPACE"),
            match status {
                GameStatus::Going => colors::BLACK,
                GameStatus::Won(_) | GameStatus::Failed(_) => colors::WHITE,
            },
        ),
    );

    response.clicked()
}

/// The keyboard widget
/// Returns which key is pressed
pub fn keyboard(
    ui: &mut egui::Ui,
    alphabet: &Alphabet,
    status: &GameStatus,
    valid: bool,
) -> Option<char> {
    let mut pressed: Option<char> = None;

    // Render the r-th row
    let mut row = |r, letters: &str| {
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
                ENTER => {
                    if enter_key(ui, status, valid, x, y) {
                        pressed = Some(c)
                    }
                }
                BACKSPACE => {
                    if backspace_key(ui, status, x, y) {
                        pressed = Some(c)
                    }
                }
                _ => {
                    if letter_key(
                        ui,
                        c,
                        &alphabet[game::get_index(c.to_ascii_lowercase())],
                        x,
                        y,
                    ) {
                        pressed = Some(c);
                    }
                }
            }
        }
    };

    row(1, "QWERTYUIOP");
    row(2, "ASDFGHJKL");
    row(3, &format!("{ENTER}ZXCVBNM{BACKSPACE}"));

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
