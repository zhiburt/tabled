//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{
    object::{Columns, Object, Rows},
    Border, Highlight, Style, TableIteratorExt,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data
        .table()
        .with(Style::modern())
        .with(Highlight::new(
            Rows::first().and(Columns::single(1)),
            Border::filled('*'),
        ))
        .to_string();

    println!("{}", table);
}
