//! This example demonstrates how [`CompactTable`] is limited to single
//! line rows.
//!
//! * Note how the multiline data is accepted, but then truncated in the display.

#![allow(unused_variables)]

use tabled::{settings::style::Style, tables::CompactTable};

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
