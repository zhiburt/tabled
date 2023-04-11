//! This example demonstrates a [`PoolTable`] usage.

use tabled::{
    settings::{Alignment, Style},
    tables::PoolTable,
};

fn main() {
    let characters = [
        "Naruto Uzumaki",
        "Kakashi Hatake",
        "Minato Namikaze",
        "Jiraiya",
        "Orochimaru",
        "Itachi Uchiha",
    ];

    let data = characters.chunks(2);

    let table = PoolTable::new(data)
        .with(Style::dots())
        .with(Alignment::center())
        .to_string();

    println!("{table}");
}
