//! The example can be run by this command
//! `cargo run --example basic`

use tabled::{settings::style::Style, tables::compact::CompactTable};

fn main() {
    let data = [
        ["Debian", "", "true"],
        ["Arch", "", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::new(data, 3, 8).with(Style::markdown());

    println!("{table}");
}
