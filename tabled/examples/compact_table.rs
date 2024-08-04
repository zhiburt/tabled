//! This example demonstrates creating a `new()` [`CompactTable`] with
//! manual specifications for column count, column widths, and border styling.
//!
//! * [`CompactTable`] is a [`Table`] alternative that trades off reduced
//!   flexibility for improved performance.

#![allow(unused_variables)]

use tabled::{settings::style::Style, tables::CompactTable};

fn main() {
    let data = [
        ["Debian", "", "true"],
        ["Arch", "", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::new(data)
        .columns(3)
        .width([7, 5, 5])
        .with(Style::markdown());

    #[cfg(feature = "std")]
    println!("{}", table.to_string());
}
