//! This example demonstrates using [`Highlight`] in combination with [`BorderColor`] to
//! frame sections of a [`Table`] with a unique background [`Color`].
//!
//! * Note how [`Highlight::colored()`] is used to accept the necessary input instead of [`Highlight::new()`].

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        style::BorderColor,
        Color, Highlight, Style,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = Table::new(data)
        .with(Style::modern())
        .with(Highlight::colored(
            Rows::first().and(Columns::single(1)),
            BorderColor::filled(Color::BG_BRIGHT_BLACK),
        ))
        .to_string();

    println!("{table}");
}
