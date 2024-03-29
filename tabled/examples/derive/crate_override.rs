//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`skip`] to omit specific fields from becoming columns in a [`Table`] display.
//!
//! * Note how [`skip`] annoys [clippy](https://doc.rust-lang.org/clippy/) with `dead_code`
//! warnings. This can be addressed with compiler overrides like `#[allow(dead_code)]`.

pub mod unknown_crate {
    pub use ::tabled::{Table, Tabled};
}

#[allow(non_camel_case_types, dead_code)]
type tabled = usize;

use unknown_crate::{Table, Tabled};

#[derive(Tabled)]
#[tabled(crate = "unknown_crate")]
struct Country<'a> {
    name: &'a str,
    capital_city: &'a str,
}

impl<'a> Country<'a> {
    fn new(name: &'a str, capital_city: &'a str) -> Self {
        Self { name, capital_city }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Kabul"),
        Country::new("Angola", "Luanda"),
        Country::new("Canada", "Ottawa"),
    ];

    let table = Table::new(data);

    println!("{table}");
}
