//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`rename_all`] to apply table-wide header formatting in a [`Table`] display.
//!
//! * Supported formatting rules include:
//!     * 'camelCase'
//!     * 'kabab-case'
//!     * 'PascalCase'
//!     * 'SCREAMING_SNAKE_CASE'
//!     * 'snake_case'
//!     * 'lowercase'
//!     * 'UPPERCASE'
//!     * 'verbatim'

use tabled::{Table, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "camelCase")]
struct Country {
    name: String,
    #[tabled(rename_all = "kebab-case")]
    capital_city: String,
    area_km2: f32,
}

impl Country {
    fn new(name: &str, city: &str, area_km2: f32) -> Self {
        Self {
            name: name.to_string(),
            capital_city: city.to_string(),
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
