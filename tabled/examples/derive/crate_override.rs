//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`skip`] to omit specific fields from becoming columns in a [`Table`] display.
//!
//! Note how [`skip`] annoys [clippy](https://doc.rust-lang.org/clippy/) with `dead_code`
//! warnings. This can be addressed with compiler overrides like `#[allow(dead_code)]`.

pub mod unknown_crate {
    pub use ::tabled::{Table, Tabled};
}

// make sure we are not using default 'tabled::*' path
#[allow(non_camel_case_types, dead_code)]
type tabled = usize;

#[derive(unknown_crate::Tabled)]
#[tabled(crate = "unknown_crate")]
struct Country {
    name: String,
    city: String,
}

impl Country {
    fn new(name: &str, city: &str) -> Self {
        Self {
            name: name.to_string(),
            city: city.to_string(),
        }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Kabul"),
        Country::new("Angola", "Luanda"),
        Country::new("Canada", "Ottawa"),
    ];

    let table = unknown_crate::Table::new(data);

    println!("{table}");
}
