//! The example can be run by this command
//! `cargo run --example split`

use std::iter::FromIterator;
use tabled::{
    col, row,
    settings::{split::Split, style::Style, Padding},
    Table,
};

fn main() {
    let table = Table::from_iter(['a'..='z']);

    let table_1 = table.clone().with(Split::column(12)).clone();
    let table_2 = table_1.clone().with(Split::column(2).zip()).to_string();
    let table_3 = table_1.clone().with(Split::column(2).concat()).to_string();
    let table_4 = table_1.clone().with(Split::row(2).zip()).to_string();
    let table_5 = table_1.clone().with(Split::row(2).concat()).to_string();

    let mut table = col![
        table,
        row![table_1, table_2, table_3, table_4, table_5]
            .with(Style::blank())
            .with(Padding::zero()),
    ];
    table.with(Style::blank());

    println!("{table}");

    let table = Table::from_iter([vec![String::new(); 26]]);

    let table_1 = table.clone().with(Split::column(12)).clone();
    let table_2 = table_1.clone().with(Split::column(2).zip()).to_string();
    let table_3 = table_1.clone().with(Split::column(2).concat()).to_string();
    let table_4 = table_1.clone().with(Split::row(2).zip()).to_string();
    let table_5 = table_1.clone().with(Split::row(2).concat()).to_string();

    let mut table = col![
        table,
        row![table_1, table_2, table_3, table_4, table_5]
            .with(Style::blank())
            .with(Padding::zero()),
    ];
    table.with(Style::blank());

    println!("{table}");
}
