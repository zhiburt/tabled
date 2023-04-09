//! This example demonstrates using the [`Disable`] [`TableOption`] to remove specific
//! cell data from a [`Table`] display.
//!
//! * ⚠️ Using [`Disable`] in combination with other [`Style`] customizations may yield unexpected results.
//! It is safest to use [`Disable`] last in a chain of alterations.

use tabled::{
    settings::{
        locator::ByColumnName,
        style::{Border, Style},
        Disable, Modify,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    based_on: &'static str,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &'static str, based_on: &'static str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name,
            based_on,
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
    table
        .with(Style::markdown())
        .with(Disable::column(ByColumnName::new("is_active")))
        .with(Modify::new(ByColumnName::new("name")).with(Border::filled('#')));

    println!("{table}");
}
