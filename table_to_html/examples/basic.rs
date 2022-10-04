//! The example can be run by this command
//! `cargo run --example basic`

use std::iter::FromIterator;

use table_to_html::HtmlTable;
use tabled::{object::Rows, Alignment, ModifyObject, Table, Tabled};

#[derive(Debug, Tabled)]
struct Distribution {
    name: &'static str,
    based_on: &'static str,
    is_active: bool,
}

impl Distribution {
    fn new(name: &'static str, base: &'static str, is_active: bool) -> Self {
        Self {
            based_on: base,
            is_active,
            name,
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
    table.with(Rows::first().modify().with(Alignment::center()));

    let html_table = HtmlTable::from(table);

    println!("{}", html_table);
}
