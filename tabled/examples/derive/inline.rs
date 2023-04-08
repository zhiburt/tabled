//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`inline`] to expand struct fields to individual columns in a [`Table`] display.
//!
//! * Note that without inlining a struct or enum field, those objects
//! must implement the [`Display`] trait as they will be represented in
//! a single column with the value of their [`ToString`] output.

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    surface_area_km2: f32,
    #[tabled(inline)]
    currency: Currency,
}

#[derive(Tabled)]
struct Currency {
    str: &'static str,
    short: &'static str,
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
            capital_city,
            surface_area_km2,
            currency: Currency {
                str: national_currency,
                short: national_currency_short,
            },
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
