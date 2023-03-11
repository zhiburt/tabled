//! The example can be run by this command
//! `cargo run --example highlight_color`

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
