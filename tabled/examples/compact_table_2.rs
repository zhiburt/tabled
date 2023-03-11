//! The example can be run by this command
//! `cargo run --example compact_table_2`

use tabled::{settings::Style, tables::compact::CompactTable};

fn main() {
    let data = [
        ["Debian", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::from(data).with(Style::psql());

    #[cfg(feature = "std")]
    println!("{}", table.to_string());
}
