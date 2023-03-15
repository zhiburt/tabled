//! The example can be run by this command
//! `cargo run --example compact_table_3`

use tabled::{settings::Style, tables::CompactTable};

fn main() {
    let data = [
        ["De\nbi\nan", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "A\nr\nc\nh", "true"],
    ];

    let table = CompactTable::from(data).with(Style::psql());

    #[cfg(feature = "std")]
    println!("{}", table.to_string());
}
