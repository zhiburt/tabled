//! The example can be run by this command
//! `cargo run --example basic`

use tabled::{object::Rows, Alignment, ModifyObject, Style, Table, Tabled};

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
        Distribution::new(
            "\u{1b}]8;;http://www.google.com\u{1b}\\google\u{1b}]8;;\u{1b}\\",
            "Arch",
            true,
            true,
        ),
        Distribution::new("Arch", "", true, true),
        Distribution::new("Debian", "", true, true),
    ];

    let table = Table::new(&data)
        .with(Style::markdown())
        .with(Rows::new(1..).modify().with(Alignment::left()));

    println!("{}", table);
}
