//! The example can be run by this command
//! `cargo run --example basic`

use std::iter::FromIterator;

use tabled::{
    settings::{alignment::Alignment, object::Rows, style::Style, Modify},
    Table, Tabled,
};

#[derive(Debug, Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
}

impl Distribution {
    fn new(name: &str, base: &str, is_active: bool) -> Self {
        Self {
            based_on: base.to_owned(),
            name: name.to_owned(),
            is_active,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Debian", "", true),
        Distribution::new("Arch", "", true),
        Distribution::new("Manjaro", "Arch", true),
    ];

    let mut table = Table::from_iter(&data);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    println!("{table}");
}
