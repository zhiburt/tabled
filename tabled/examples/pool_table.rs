//! This example demonstrates a [`PoolTable`] usage.

use tabled::{
    settings::{Alignment, Style},
    tables::{PoolTable, TableValue},
};

fn main() {
    let data = vec![
        vec!["Hello World", "Hello World", "Hello World"],
        vec!["Hello", "", "Hello"],
        vec!["W", "o", "r", "l", "d"],
    ];

    let data = TableValue::Column(
        data.into_iter()
            .map(|row| {
                TableValue::Row(
                    row.into_iter()
                        .map(|text| TableValue::Cell(text.to_owned()))
                        .collect(),
                )
            })
            .collect(),
    );

    let table = PoolTable::from(data)
        .with(Style::modern())
        .with(Alignment::center())
        .to_string();

    println!("{table}");
}
