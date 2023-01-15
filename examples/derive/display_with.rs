//! The example can be run by this command
//! `cargo run --example display_with`

use std::borrow::Cow;

use tabled::{TableIteratorExt, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "camelCase")]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    #[tabled(display_with("display_perimeter", args))]
    surface_area_km2: f32,
    #[tabled(display_with = "str::to_lowercase")]
    national_currency: &'static str,
    national_currency_short: &'static str,
}

fn display_perimeter(country: &Country) -> Cow<'_, str> {
    if country.surface_area_km2 > 1_000_000.0 {
        "Very Big Land".into()
    } else {
        "Big Land".into()
    }
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
