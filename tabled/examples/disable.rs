//! This example demonstrates using the [`Disable`] [`TableOption`] to remove specific
//! cell data from a [`Table`] display.
//!
//! * ⚠️ Using [`Disable`] in combination with other [`Style`] customizations may yield unexpected results.
//! It is safest to use [`Disable`] last in a chain of alterations.

use tabled::{
    settings::{location::ByColumnName, Disable},
    Table, Tabled,
};

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
        Distribution::new("Debian", "", true, true),
        Distribution::new("Arch", "", true, true),
        Distribution::new("Manjaro", "Arch", true, true),
    ];

    let mut table = Table::new(data);
    table.with(Disable::column(ByColumnName::new("is_active")));

    println!("{table}");
}
