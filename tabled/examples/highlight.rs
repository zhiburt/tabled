//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{
    settings::{
        highlight::Highlight,
        object::{Columns, Object, Rows},
        style::{Border, Style},
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
