//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{
    style::{Border, Style},
    Cell, Highlight, TableIteratorExt,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data
        .table()
        .with(Style::modern())
        .with(Highlight::new(Cell(2, 1), Border::filled('*')));

    println!("{}", table);
}
