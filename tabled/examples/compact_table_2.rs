//! The example can be run by this command
//! `cargo run --example compact_table_2`

use tabled::{settings::style::Style, tables::compact::CompactTable};

fn main() {
    let data = [
        ["Debian", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::from(data).with(Style::psql());

    println!("{table}");
}
