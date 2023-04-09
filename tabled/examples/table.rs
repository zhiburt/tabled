//! This example demonstrates the fundemental qualities of the [crate](https://crates.io/crates/tabled) [`tabled`].
//!
//! * [`tabled`] is powered by convenient [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros)
//! like [`Tabled`]; a deriveable trait that allows your custom
//! structs and enums to be represented with [`tabled`]'s powerful suite of features.
//!
//! * [`Table`] is the root object to which all of [`tabled`]'s implementation
//! tools guide you, and from which all of its customization options derive value.
//! The READMEs, examples, and docs found in this project will show you many dozens
//! of ways you can build tables, and show you how to use the available tools
//! to build the data representations that best fit your needs.
//!
//! * [`Table::with()`] plays a central role in giving the user control over their
//! displays. A majority of [`Table`] customizations are implemented through this highly
//! dynamic function. A few [`TableOptions`](TableOption) include:
//!     * [`Style`]
//!     * [`Modify`]
//!     * [`Alignment`]
//!     * [`Padding`]

use tabled::{
    settings::{object::Rows, Alignment, Modify, Style},
    Table, Tabled,
};

#[derive(Debug, Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
}

impl Distribution {
    fn new(name: &str, base: &str, is_active: bool) -> Self {
        Self {
            based_on: base.to_owned(),
            name: name.to_owned(),
            is_active,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Debian", "", true),
        Distribution::new("Arch", "", true),
        Distribution::new("Manjaro", "Arch", true),
    ];

    let mut table = Table::new(data);
    table
        .with(Style::markdown())
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    println!("{table}");
}
