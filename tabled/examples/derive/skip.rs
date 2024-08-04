//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`skip`] to omit specific fields from becoming columns in a [`Table`] display.
//!
//! Note how [`skip`] annoys [clippy](https://doc.rust-lang.org/clippy/) with `dead_code`
//! warnings. This can be addressed with compiler overrides like `#[allow(dead_code)]`.

#![allow(dead_code)]

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: String,
    #[tabled(skip)]
    capital: String,
    #[tabled(skip)]
    area_km2: f32,
}

impl Country {
    fn new(name: &str, capital: &str, area: f32) -> Self {
        Self {
            name: name.to_string(),
            capital: capital.to_string(),
            area_km2: area,
        }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Kabul", 652867.0),
        Country::new("Angola", "Luanda", 1246700.0),
        Country::new("Canada", "Ottawa", 9984670.0),
    ];

    let table = Table::new(data);

    println!("{table}");
}
