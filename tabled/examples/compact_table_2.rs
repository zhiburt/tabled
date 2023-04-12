//! This example demonstrates creating a [`CompactTable`] `from()` a
//! multidimensional array.
//!
//! * Note how [`CompactTable::from()`] inherits the lengths of the nested arrays
//! as typed definitions through [const generics](https://practice.rs/generics-traits/const-generics.html).

use tabled::{settings::Style, tables::CompactTable};

fn main() {
    let data = [
        ["Debian", "1.1.1.1", "true"],
        ["Arch", "127.1.1.1", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let _table = CompactTable::from(data).with(Style::psql());

    #[cfg(feature = "std")]
    println!("{}", _table.to_string());
}
