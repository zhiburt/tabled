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
//!   augmented fields natural value entirely. Even an entire object can be passed as context with `self`.

use std::borrow::Cow;

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: String,
    #[tabled(display_with = "str::to_uppercase")]
    capital: String,
    #[tabled(display_with("display_perimeter", self))]
    area_km2: f32,
}

impl Country {
    fn new(name: &str, capital: &str, area_km2: f32) -> Self {
        Self {
            name: name.to_string(),
            capital: capital.to_string(),
            area_km2,
        }
    }
}

fn display_perimeter(country: &Country) -> Cow<'_, str> {
    if country.area_km2 > 1_000_000.0 {
        "BIG".into()
    } else {
        "small".into()
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
