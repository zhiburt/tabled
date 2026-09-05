// Clearly clippy false positive.
#![allow(clippy::single_range_in_vec_init)]

use std::iter::FromIterator;

use tabled::Table;

fn main() {
    let table = Table::from_iter(['a'..='z']);
    println!("{table}");
}
