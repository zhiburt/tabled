//! This example demonstrates instantiating a [`Table`] from an [`IntoIterator`] compliant object.
//!
//! * Note how [`Range`] [expression syntax](https://doc.rust-lang.org/reference/expressions/range-expr.html)
//! is used to idiomatically represent the English alphabet.

use std::iter::FromIterator;

use tabled::Table;

fn main() {
    let table = Table::from_iter(['a'..='z']);
    println!("{table}");
}
