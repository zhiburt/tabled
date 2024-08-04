//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`order`] to relocate fields to specified indexes in a [`Table`] display.
//!
//! * By default, [`Table`] columns are shown in the same ordered they are
//!   defined in the deriving struct/enum definition.

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: String,
    capital: String,
    #[tabled(order = 0)]
    area_km2: f32,
}

impl Country {
    fn new(name: &str, city: &str, area_km2: f32) -> Self {
        Self {
            name: name.to_string(),
            capital: city.to_string(),
            area_km2,
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
