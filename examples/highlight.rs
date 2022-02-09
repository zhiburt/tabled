//! The example can be run by this command
//! `cargo run --example highlight`

use tabled::{Border, Highlight, Style, TableIteratorExt};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data.table().with(Style::modern()).with(Highlight::cell(
        2,
        1,
        Border::full('*', '*', '*', '*', '*', '*', '*', '*'),
    ));

    println!("{}", table);
}
