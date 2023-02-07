//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{
    color::Color,
    highlight::Highlight,
    object::{Columns, Object, Rows},
    style::BorderColor,
    Style, TableIteratorExt,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data
        .table()
        .with(Style::modern())
        .with(Highlight::colored(
            Rows::first().and(Columns::single(1)),
            BorderColor::filled(Color::BG_BRIGHT_BLACK),
        ))
        .to_string();

    println!("{}", table);
}
