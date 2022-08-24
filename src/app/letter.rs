use egui::{Align2, Color32, FontFamily, Pos2, Rect, Sense, Vec2};

use crate::game::LetterStatus;

use super::colors;
use super::keyboard;
use super::metrics;
use super::utils;

/// A letter consists of a char and a status, for rendering in UI
/// The letter could be None, standing for a state of "not yet input"
pub struct Letter {
    pub letter: Option<char>,
    pub status: LetterStatus,
}

impl Letter {
    /// Get the stroke color for a letter
    fn get_stroke_color(&self, dark: bool) -> Color32 {
        if dark {
            if let Some(_) = self.letter {
                match self.status {
                    LetterStatus::Unknown => colors::DARK_MODE_GRAY,
                    LetterStatus::Red => colors::DARK_MODE_DARK_GRAY,
                    LetterStatus::Yellow => colors::DARK_MODE_YELLOW,
                    LetterStatus::Green => colors::DARK_MODE_GREEN,
                }
            } else {
                colors::DARK_MODE_DARK_GRAY
            }
        } else {
            if let Some(_) = self.letter {
                match self.status {
                    LetterStatus::Unknown => colors::DARK_GRAY,
                    LetterStatus::Red => colors::DARK_GRAY,
                    LetterStatus::Yellow => colors::YELLOW,
                    LetterStatus::Green => colors::GREEN,
                }
            } else {
                colors::GRAY
            }
        }
    }

    /// Get the fill color for a letter
    fn get_fill_color(&self, dark: bool) -> Color32 {
        if dark {
            if let Some(_) = self.letter {
                match self.status {
                    LetterStatus::Unknown => colors::DARK_MODE_BLACK,
                    LetterStatus::Red => colors::DARK_MODE_DARK_GRAY,
                    LetterStatus::Yellow => colors::DARK_MODE_YELLOW,
                    LetterStatus::Green => colors::DARK_MODE_GREEN,
                }
            } else {
                colors::DARK_MODE_BLACK
            }
        } else {
            if let Some(_) = self.letter {
                match self.status {
                    LetterStatus::Unknown => colors::WHITE,
                    LetterStatus::Red => colors::DARK_GRAY,
                    LetterStatus::Yellow => colors::YELLOW,
                    LetterStatus::Green => colors::GREEN,
                }
            } else {
                colors::WHITE
            }
        }
    }

    /// Get the text color for a letter
    fn get_text_color(&self, dark: bool) -> Color32 {
        if dark {
            colors::DARK_MODE_WHITE
        } else {
            if self.status == LetterStatus::Unknown {
                colors::BLACK
            } else {
                colors::WHITE
            }
        }
    }
}

/// How many height does the grid expects to occupy
fn get_expected_height(ui: &egui::Ui) -> f32 {
    (metrics::ROWS as f32 * (metrics::LETTER_BOX_SIZE + metrics::LETTER_MARGIN)
        - metrics::LETTER_MARGIN)
        * keyboard::get_keyboard_size_factor(ui)
}

/// How many height is available for use after allocation to header and keyboard
fn get_available_height(ui: &egui::Ui) -> f32 {
    let keyboard_height = (metrics::KEYBOARD_ROWS as f32
        * (metrics::KEY_HEIGHT + metrics::KEY_V_MARGIN)
        + metrics::KEY_V_MARGIN)
        * keyboard::get_keyboard_size_factor(ui);
    utils::get_screen_height(ui)
        - metrics::HEADER_HEIGHT
        - metrics::HEADING_GRID_GAP
        - keyboard_height
}

/// Responsively scale the size of a letter to make sure contents don't cover each other
fn get_letter_size_factor(ui: &egui::Ui) -> f32 {
    let expected = get_expected_height(ui);
    let available = get_available_height(ui);
    // Scale the letter grid to fit in available space
    let mut factor = (available / expected).min(1.0);
    // On small width device (like a phone)
    if keyboard::get_keyboard_size_factor(ui) != 1.0 {
        factor *= keyboard::get_keyboard_size_factor(ui);
    }
    factor
}

/// The letter widget
pub fn letter(ui: &mut egui::Ui, dark: bool, row: i32, column: i32, letter: &Letter) {
    // Assert parameters row and column are in correct range
    assert!((0..metrics::ROWS).contains(&row));
    assert!((0..metrics::COLUMNS).contains(&column));

    // Compute actual metrics
    let factor = get_letter_size_factor(ui);
    let box_size = factor * metrics::LETTER_BOX_SIZE;
    let margin = factor * metrics::LETTER_MARGIN;
    let font_size = factor * metrics::LETTER_FONT_SIZE;

    // Compute x and y position where we put the letter
    let x = utils::get_screen_width(ui) / 2.0 - (box_size + margin) * 2.0 - box_size / 2.0
        + (box_size + margin) * column as f32;
    let y = metrics::HEADER_HEIGHT
        + metrics::HEADING_GRID_GAP
        + (box_size + margin) * row as f32
        + if get_available_height(ui) > get_expected_height(ui) {
            (get_available_height(ui) - get_expected_height(ui)) / 2.0
        } else {
            0.0
        };

    // Painting rect
    let rect = Rect::from_min_size(
        Pos2 { x, y: y as f32 },
        Vec2 {
            x: box_size,
            y: box_size,
        },
    );

    // Allocate space for the widget
    ui.allocate_rect(rect, Sense::hover());

    // Compute animated colors
    let fill_color = utils::animate_color(
        ui.ctx(),
        format!("letter{row}{column}s"),
        letter.get_fill_color(dark),
    );
    let stroke_color = utils::animate_color(
        ui.ctx(),
        format!("letter{row}{column}f"),
        letter.get_stroke_color(dark),
    );
    let text_color = utils::animate_color(
        ui.ctx(),
        format!("letter{row}{column}t"),
        letter.get_text_color(dark),
    );

    // Paint the rectangle area
    ui.painter().rect(
        rect,
        0.0,
        fill_color,
        (metrics::LETTER_STROKE, stroke_color),
    );

    // Paint the text
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        letter.letter.unwrap_or(' '),
        egui::FontId {
            size: font_size,
            family: FontFamily::Name("SF".into()),
        },
        text_color,
    );
}
