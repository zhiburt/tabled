//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//! specifications. This is helpful for organizing extensive [`Table`] configurations.

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{style::Style, themes::Colorization, Color},
};

fn main() {
    let board = [
        ["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"],
        ["♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟"],
        [" ", " ", " ", " ", " ", " ", " ", " "],
        [" ", " ", " ", " ", " ", " ", " ", " "],
        [" ", " ", " ", " ", " ", " ", " ", " "],
        [" ", " ", " ", " ", " ", " ", " ", " "],
        ["♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙"],
        ["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"],
    ];

    let color_white = Color::BG_WHITE | Color::FG_BLACK;
    let color_black = Color::FG_WHITE | Color::BG_BLACK;

    let mut table = Builder::from_iter(board).build();
    table
        .with(Style::empty())
        .with(Colorization::chess(color_white, color_black));

    println!("{table}");
}
