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
    name: &'static str,
    capital_city: &'static str,
    surface_area_km2: f32,
    #[tabled(rename_all = "kebab-case")]
    national_currency: &'static str,
    #[tabled(rename_all = "kebab-case")]
    national_currency_short: &'static str,
}

impl Country {
    fn new(
        name: &'static str,
        national_currency: &'static str,
        national_currency_short: &'static str,
        capital_city: &'static str,
        surface_area_km2: f32,
    ) -> Self {
        Self {
            name,
            national_currency,
            national_currency_short,
            capital_city,
            surface_area_km2,
        }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Afghani", "AFN", "Kabul", 652867.0),
        Country::new("Angola", "Kwanza", "AOA", "Luanda", 1246700.0),
        Country::new("Canada", "Canadian Dollar", "CAD", "Ottawa", 9984670.0),
    ];

    let table = Table::new(data);

    println!("{table}");
}
