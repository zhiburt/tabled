//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`inline`] to expand struct fields to individual columns in a [`Table`] display.
//!
//! * Note that without inlining a struct or enum field, those objects
//!   must implement the [`Display`] trait as they will be represented in
//!   a single column with the value of their [`ToString`] output.

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: String,
    #[tabled(inline)]
    currency: Currency,
    area_km2: f32,
}

#[derive(Tabled)]
struct Currency {
    currency: String,
    currency_short: String,
}

impl Country {
    fn new(name: &str, currency: &str, currency_short: &str, area_km2: f32) -> Self {
        Self {
            name: name.to_string(),
            area_km2,
            currency: Currency {
                currency: currency.to_string(),
                currency_short: currency_short.to_string(),
            },
        }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Afghani", "AFN", 652867.0),
        Country::new("Angola", "Kwanza", "AOA", 1246700.0),
        Country::new("Canada", "Canadian Dollar", "CAD", 9984670.0),
    ];

    let table = Table::new(data);

    println!("{table}");
}
