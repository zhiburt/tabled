//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`skip`] to omit specific fields from becoming columns in a [`Table`] display.
//!
//! * Note how [`skip`] annoys [clippy](https://doc.rust-lang.org/clippy/) with `dead_code`
//! warnings. This can be addressed with compiler overrides like `#[allow(dead_code)]`.

use tabled::{Table, Tabled};

#[allow(dead_code)]
#[derive(Tabled)]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    #[tabled(skip)]
    surface_area_km2: f32,
    national_currency: &'static str,
    #[tabled(skip)]
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
