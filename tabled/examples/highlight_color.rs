//! This example demonstrates using [`Highlight`] in combination with [`BorderColor`] to
//! frame sections of a [`Table`] with a unique background [`Color`].
//!
//! * Note how [`Highlight::colored()`] is used to accept the necessary input instead of [`Highlight::new()`].

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        Color, Highlight, Style,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let target = Rows::first().and(Columns::single(1));
    let color = Color::BG_BRIGHT_BLACK;

    let mut table = Table::new(data);
    table.with(Style::modern());
    table.with(Highlight::colored(target, color));

    println!("{table}");
}
