//! This example demonstrates using the [`Margin`] [`TableOption`] to buffer space
//! around a [`Table`] display.
//!
//! * Note how the [`Margin::fill()`] function allows for overriding the default whitespace
//! with any [`char`].

use tabled::{
    settings::{Margin, Style},
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = Table::new(data)
        .with(Style::re_structured_text())
        .with(Margin::new(4, 3, 2, 1).fill('<', '>', 'v', '^'))
        .to_string();

    println!("{table}");
}
