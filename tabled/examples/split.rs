//! The example can be run by this command
//! `cargo run --example split`

use std::iter::FromIterator;
use tabled::{
    col, row,
    settings::{
        split::{Behavior, Split},
        style::Style,
    },
    Table,
};

fn main() {
    let mut table = Table::from_iter(['a'..='z']);

    table.with(Split::column(6));

    let mut table_1 = table.clone();
    let mut table_2 = table.clone();
    let mut table_3 = table.clone();
    let mut table_4 = table.clone();

    table_1.with(Split::column(2));
    table_2.with(Split::column(2).set_behavior(Behavior::Append));
    table_3.with(Split::row(2));
    table_4.with(Split::row(2).set_behavior(Behavior::Append));

    println!(
        "{}",
        col![
            row![table, table_1, table_2].with(Style::blank()),
            table_3,
            table_4
        ]
        .with(Style::blank())
    );
}
