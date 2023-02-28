//! The example can be run by this command
//! `cargo run --example alphabet`

use std::iter::FromIterator;

use tabled::Table;

fn main() {
    let table = Table::from_iter(['a'..='z']);
    println!("{table}");
}
