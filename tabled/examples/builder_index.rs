//! This example demonstrates evolving the standard [`Builder`] to an [`IndexBuilder`],
//! and then manipulating the constructing table with a newly prepended index column.
//!
//! * An [`IndexBuilder`] is capable of several useful manipulations, including:
//!     * Giving the new index column a name
//!     * Transposing the index column around a table
//!     * Choosing a location for the new index column besides 0; the default
//!
//! * Note that like with any builder pattern the [`IndexBuilder::build()`] function
//! is necessary to produce a displayable [`Table`].

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: &'static str,
    based_on: &'static str,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &'static str, based_on: &'static str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name,
            based_on,
            is_active,
            is_cool,
        }
    }
}

fn main() {
    let data = [
        Distribution::new("Manjaro", "Arch", true, true),
        Distribution::new("Arch", "None", true, true),
        Distribution::new("Debian", "None", true, true),
    ];

    let mut table = Table::builder(data)
        .index()
        .column(0)
        .name(None)
        .transpose()
        .build();

    table.with(Style::modern());

    println!("{table}");
}
