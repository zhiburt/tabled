//! The example can be run by this command
//! `cargo run --example disable`

use tabled::{disable::ByColumnName, Border, Disable, ModifyObject, Style, Table, Tabled};

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

    let table = Table::new(&data)
        .with(Style::markdown())
        .with(Disable::column(ByColumnName::new("is_active")))
        .with(ByColumnName::new("name").modify().with(Border::filled('#')));

    println!("{}", table);
}
