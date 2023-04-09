//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`display_with`] to seamlessly augment field representations in a [`Table`] display.
//!
//! * [`display_with`] functions act as transformers during [`Table`] instantiation.
//!
//! * Note how [`display_with`] works with [std] and custom functions alike.
//!
//! * [`display_with`] attributes can be constructed in two ways (shown below).
//!
//! * Attribute arguments can be directly overridden with static values, effectively ignoring the
//! augmented fields natural value entirely. Even an entire object can be passed as context with `self`.

use std::borrow::Cow;

use tabled::{Table, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "camelCase")]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    #[tabled(display_with("display_perimeter", self))]
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

    let table = Table::new(data);

    println!("{table}");
}
