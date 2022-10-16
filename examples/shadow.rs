//! The example can be run by this command
//! `cargo run --example basic`

use std::iter::FromIterator;

use tabled::{object::Rows, shadow::Shadow, Alignment, ModifyObject, Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &str, based_on: &str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name: name.to_string(),
            based_on: based_on.to_string(),
            is_active,
            is_cool,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Manjaro", "Arch", true, true),
        Distribution::new("Arch", "", true, true),
        Distribution::new("Debian", "", true, true),
    ];

    let mut table = Table::from_iter(&data);
    table
        .with(Style::ascii())
        .with(Rows::first().modify().with(Alignment::center()))
        .with(Shadow::new(2));

    println!("{}", table);
}
