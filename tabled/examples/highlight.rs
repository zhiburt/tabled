//! This example demonstrates using the [`Highlight`] [`TableOption`] to
//! decorate sections of a [`Table`] with a unique [`Border`].
//!
//! * Note how [`Highlight`] arguments can be chained together to
//! create cross-sections and non-symmetrical shapes.

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        style::{Border, Style},
        Highlight,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = Table::new(data)
        .with(Style::modern())
        .with(Highlight::new(
            Rows::first().and(Columns::single(1)),
            Border::filled('*'),
        ))
        .to_string();

    println!("{table}");
}
