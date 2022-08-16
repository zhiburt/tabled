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

fn main() {
    let data = [
        Distribution {
            name: "Debian",
            based_on: "",
            is_active: true,
            is_cool: true,
        },
        Distribution {
            name: "Arch",
            based_on: "",
            is_active: true,
            is_cool: true,
        },
        Distribution {
            name: "Manjaro",
            based_on: "Arch",
            is_active: true,
            is_cool: true,
        },
    ];

    let table = Table::new(&data)
        .with(Style::markdown())
        .with(Rows::new(1..).modify().with(Alignment::left()));

    println!("{}", table);
}
