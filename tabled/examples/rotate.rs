//! This example demonstrates using the [`Rotate`] [`TableOption`] to rotate the cells
//! of a [`Table`].
//!
//! * [`Rotate`] supports four motions:
//!     * `Left` | 90 degree shift
//!     * `Right` | 90 degree shift
//!     * `Top` & `Bottom` | Reverse row order

use tabled::{settings::Rotate, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    distribution: &'static str,
    link: &'static str,
}

impl Linux {
    fn new(id: u8, distribution: &'static str, link: &'static str) -> Self {
        Self {
            id,
            distribution,
            link,
        }
    }
}

fn main() {
    let data = vec![
        Linux::new(0, "Fedora", "https://getfedora.org/"),
        Linux::new(2, "OpenSUSE", "https://www.opensuse.org/"),
        Linux::new(3, "Endeavouros", "https://endeavouros.com/"),
    ];

    let table = Table::new(data).with(Rotate::Left).to_string();

    println!("{table}");
}
