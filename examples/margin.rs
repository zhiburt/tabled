//! The example can be run by this command
//! `cargo run --example margin`

use tabled::{Margin, Style, TableIteratorExt};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let table = data
        .table()
        .with(Style::re_structured_text())
        .with(Margin::new(4, 3, 2, 1).set_fill('<', '>', 'v', '^'));

    println!("{}", table);
}
