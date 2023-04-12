//! This example demonstrates how [`CompactTable`] is limited to single
//! line rows.
//!
//! * Note how the multiline data is accepted, but then truncated in the display.

use tabled::{settings::Style, tables::CompactTable};

fn main() {
    let data = [
        ["De\nbi\nan", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "A\nr\nc\nh", "true"],
    ];

    let _table = CompactTable::from(data).with(Style::psql());

    #[cfg(feature = "std")]
    println!("{}", _table.to_string());
}
