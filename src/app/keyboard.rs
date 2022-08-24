use egui::{Align2, Color32, CursorIcon, FontFamily, Key, Modifiers, Pos2, Rect, Sense, Vec2};

use crate::game::{self, Alphabet, GameStatus, LetterStatus};

use super::{colors, metrics, utils};

pub const ENTER: char = '\n';
pub const BACKSPACE: char = '\x08';

/// How many width does the keyboard expects to occupy
fn get_expected_width() -> f32 {
    (metrics::KEY_WIDTH + metrics::KEY_H_MARGIN) * 10.0 - metrics::KEY_H_MARGIN
        + metrics::PADDING * 2.0
}

/// How many width is available for use
fn get_available_width(ui: &egui::Ui) -> f32 {
    utils::get_screen_width(ui)
}

/// Responsively scale the size of the keyboard to make sure it fits in the screen
pub fn get_keyboard_size_factor(ui: &egui::Ui) -> f32 {
    let expected = get_expected_width();
    let available = get_available_width(ui);
    // Scale the letter grid to fit in available space
    return (available / expected).min(1.0);
}

fn get_key_fill_color(dark: bool, status: &LetterStatus) -> Color32 {
    if dark {
        match status {
            LetterStatus::Unknown => colors::DARK_MODE_GRAY,
            LetterStatus::Red => colors::DARK_MODE_DARK_GRAY,
            LetterStatus::Yellow => colors::DARK_MODE_YELLOW,
            LetterStatus::Green => colors::DARK_MODE_GREEN,
        }
    } else {
        match status {
            LetterStatus::Unknown => colors::GRAY,
            LetterStatus::Red => colors::DARK_GRAY,
            LetterStatus::Yellow => colors::YELLOW,
            LetterStatus::Green => colors::GREEN,
        }
    }
}

fn get_key_text_color(dark: bool, status: &LetterStatus) -> Color32 {
    if dark {
        colors::DARK_MODE_WHITE
    } else {
        match status {
            LetterStatus::Unknown => colors::BLACK,
            _ => colors::WHITE,
        }
    }
}

fn get_enter_key_fill_color(dark: bool, status: &GameStatus, valid: bool) -> Color32 {
    if dark {
        match status {
            GameStatus::Going => {
                if valid {
                    colors::DARK_MODE_GRAY
                } else {
                    colors::DARK_MODE_DARK_GRAY
                }
            }
            GameStatus::Won(_) => colors::DARK_MODE_GREEN,
            GameStatus::Failed(_) => colors::DARK_MODE_YELLOW,
        }
    } else {
        match status {
            GameStatus::Going => {
                if valid {
                    colors::GRAY
                } else {
                    colors::DARK_GRAY
                }
            }
            GameStatus::Won(_) => colors::GREEN,
            GameStatus::Failed(_) => colors::YELLOW,
        }
    }
}

fn get_enter_key_text_color(dark: bool, status: &GameStatus, valid: bool) -> Color32 {
    if dark {
        match status {
            GameStatus::Going => {
                if valid {
                    colors::DARK_MODE_WHITE
                } else {
                    colors::DARK_MODE_GRAY
                }
            }
            GameStatus::Won(_) | GameStatus::Failed(_) => colors::DARK_MODE_WHITE,
        }
    } else {
        match status {
            GameStatus::Going => {
                if valid {
                    colors::BLACK
                } else {
                    colors::GRAY
                }
            }
            GameStatus::Won(_) | GameStatus::Failed(_) => colors::WHITE,
        }
    }
}

fn get_backspace_key_fill_color(dark: bool, status: &GameStatus) -> Color32 {
    if dark {
        match status {
            GameStatus::Going => colors::DARK_MODE_GRAY,
            GameStatus::Won(_) => colors::DARK_MODE_GREEN,
            GameStatus::Failed(_) => colors::DARK_MODE_YELLOW,
        }
    } else {
        match status {
            GameStatus::Going => colors::GRAY,
            GameStatus::Won(_) => colors::GREEN,
            GameStatus::Failed(_) => colors::YELLOW,
        }
    }
}

fn get_backspace_key_text_color(dark: bool, status: &GameStatus) -> Color32 {
    if dark {
        colors::DARK_MODE_WHITE
    } else {
        match status {
            GameStatus::Going => colors::BLACK,
            GameStatus::Won(_) | GameStatus::Failed(_) => colors::WHITE,
        }
    }
}

/// A key in the keyboard widget, returns whether this key is clicked
fn letter_key(
    ui: &mut egui::Ui,
    dark: bool,
    c: char,
    status: &LetterStatus,
    x: f32,
    y: f32,
) -> bool {
    let factor = get_keyboard_size_factor(ui);

    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH * factor,
            y: metrics::KEY_HEIGHT * factor,
        },
    );

    // We need to sense click event
    let mut response = ui.allocate_rect(rect, Sense::click());

    // Render different colors depending on cursor state
    let color = {
        let mut color = get_key_fill_color(dark, status);
        color = utils::animate_color(ui.ctx(), format!("key{c}f"), color);
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
            size: metrics::KEY_FONT_SIZE * factor,
            family: FontFamily::Name("SF".into()),
        },
        utils::animate_color(
            ui.ctx(),
            format!("key{c}t"),
            get_key_text_color(dark, status),
        ),
    );

    response.clicked()
}

/// Enter key, meanwhile shows a message when game over
fn enter_key(
    ui: &mut egui::Ui,
    dark: bool,
    status: &GameStatus,
    valid: bool,
    x: f32,
    y: f32,
) -> bool {
    let factor = get_keyboard_size_factor(ui);

    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH_LARGE * factor,
            y: metrics::KEY_HEIGHT * factor,
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
        let mut color = get_enter_key_fill_color(dark, status, valid);
        color = utils::animate_color(ui.ctx(), format!("key{}f", "ENTER"), color);
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
            GameStatus::Going => "ENTER",
            GameStatus::Won(_) => "Bravo!",
            GameStatus::Failed(answer) => answer,
        },
        egui::FontId {
            size: metrics::KEY_FONT_SIZE * factor,
            family: FontFamily::Name("SF".into()),
        },
        utils::animate_color(
            ui.ctx(),
            format!("key{}t", "ENTER"),
            get_enter_key_text_color(dark, status, valid),
        ),
    );

    response.clicked()
}

/// Backspace key, meanwhile shows a message when game over
fn backspace_key(ui: &mut egui::Ui, dark: bool, status: &GameStatus, x: f32, y: f32) -> bool {
    let factor = get_keyboard_size_factor(ui);

    // Widget rect
    let rect = Rect::from_min_size(
        Pos2 { x, y },
        Vec2 {
            x: metrics::KEY_WIDTH_LARGE * factor,
            y: metrics::KEY_HEIGHT * factor,
        },
    );

    // We need to sense click event
    let mut response = ui.allocate_rect(rect, Sense::click());

    // Render different colors depending on cursor state
    let color = {
        let mut color = get_backspace_key_fill_color(dark, status);
        color = utils::animate_color(ui.ctx(), format!("key{}f", "BACKSPACE"), color);
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
                * factor
                * if status == &GameStatus::Going {
                    1.5
                } else {
                    0.8
                },
            family: FontFamily::Name("SF".into()),
        },
        utils::animate_color(
            ui.ctx(),
            format!("key{}t", "BACKSPACE"),
            get_backspace_key_text_color(dark, status),
        ),
    );

    response.clicked()
}

/// The keyboard widget
/// Returns which key is pressed
pub fn keyboard(
    ui: &mut egui::Ui,
    dark: bool,
    alphabet: &Alphabet,
    status: &GameStatus,
    valid: bool,
) -> Option<char> {
    let mut pressed: Option<char> = None;

    let factor = get_keyboard_size_factor(ui);

    // Render the r-th row
    let mut row = |r, letters: &str| {
        let mut row_width = metrics::PADDING * 2.0
            + letters.len() as f32 * (metrics::KEY_WIDTH + metrics::KEY_H_MARGIN) * factor
            - metrics::KEY_H_MARGIN * factor;
        if letters.contains(ENTER) {
            row_width += (metrics::KEY_WIDTH_LARGE - metrics::KEY_WIDTH) * factor;
        }
        if letters.contains(BACKSPACE) {
            row_width += (metrics::KEY_WIDTH_LARGE - metrics::KEY_WIDTH) * factor;
        }
        let mut allocated_width = metrics::PADDING;
        for c in letters.chars() {
            let x = {
                let x = utils::get_screen_width(ui) / 2.0 - row_width / 2.0 + allocated_width;
                allocated_width += metrics::KEY_H_MARGIN * factor
                    + if c == ENTER {
                        metrics::KEY_WIDTH_LARGE
                    } else {
                        metrics::KEY_WIDTH
                    } * factor;
                x
            };
            let y = utils::get_screen_height(ui)
                - (metrics::KEY_HEIGHT + metrics::KEY_V_MARGIN) * (3.0 + 1.0 - r as f32) * factor;
            // Detect keystroke
            match c {
                ENTER => {
                    if enter_key(ui, dark, status, valid, x, y) {
                        pressed = Some(c)
                    }
                }
                BACKSPACE => {
                    if backspace_key(ui, dark, status, x, y) {
                        pressed = Some(c)
                    }
                }
                _ => {
                    if letter_key(ui, dark, c, &alphabet[game::get_index(c)], x, y) {
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
