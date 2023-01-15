//! The example can be run by this command
//! `cargo run --example order`

use tabled::{TableIteratorExt, Tabled};

#[derive(Tabled)]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    surface_area_km2: f32,
    #[tabled(order = 1)]
    national_currency: &'static str,
    #[tabled(order = 2)]
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

    let table = data.table();

    println!("{table}");
}
