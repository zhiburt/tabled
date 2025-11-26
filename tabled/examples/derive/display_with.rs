//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`display`] to seamlessly augment field representations in a [`Table`] display.
//!
//! * [`display`] functions act as transformers during [`Table`] instantiation.
//!
//! * Note how [`display`] works with [std] and custom functions alike.
//!
//! * [`display`] attributes can be constructed in two ways (shown below).
//!
//! * Attribute arguments can be directly overridden with static values, effectively ignoring the
//!   augmented fields natural value entirely. Even an entire object can be passed as context with `self`.

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Country {
    name: String,
    #[tabled(display = "str::to_uppercase")]
    capital: String,
    #[tabled(display("perimeter", self, false))]
    area_km2: f32,
    #[tabled(display("tabled::derive::display::option", "unknown"))]
    currency: Option<String>,
}

fn perimeter(area: &f32, country: &Country, _milies: bool) -> String {
    let is_big = *area > 1_000_000.0f32;
    let big_sign = if is_big { "B" } else { "" };
    format!("{} {}", country.area_km2, big_sign)
}

fn main() {
    let data = [
        Country {
            name: String::from("Afghanistan"),
            capital: String::from("Kabul"),
            area_km2: 652867.0,
            currency: Some(String::from("Afghan afghani (AFN)")),
        },
        Country {
            name: String::from("Angola"),
            capital: String::from("Luanda"),
            area_km2: 1246700.0,
            currency: None,
        },
        Country {
            name: String::from("Canada"),
            capital: String::from("Ottawa"),
            area_km2: 9984670.0,
            currency: None,
        },
    ];

    let table = Table::new(data);

    println!("{table}");
}
