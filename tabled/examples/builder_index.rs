//! This example demonstrates evolving the standard [`Builder`] to an [`IndexBuilder`],
//! and then manipulating the constructing table with a newly prepended index column.
//!
//! * An [`IndexBuilder`] is capable of several useful manipulations, including:
//!     * Giving the new index column a name
//!     * Transposing the index column around a table
//!     * Choosing a location for the new index column besides 0; the default
//!
//! * Note that like with any builder pattern the [`IndexBuilder::build()`] function
//!   is necessary to produce a displayable [`Table`].

use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Distribution {
    name: String,
    based_on: String,
    is_active: bool,
    is_cool: bool,
}

impl Distribution {
    fn new(name: &str, based: &str, is_active: bool, is_cool: bool) -> Self {
        Self {
            name: name.to_string(),
            based_on: based.to_string(),
            is_active,
            is_cool,
        }
    }
}

fn main() {
    let data = vec![
        Distribution::new("Manjaro", "Arch", true, true),
        Distribution::new("Arch", "None", true, true),
        Distribution::new("Debian", "None", true, true),
    ];

    let mut table = Table::builder(data)
        .index()
        .column(0)
        .transpose()
        .name(None)
        .build();

    table.with(Style::modern_rounded());

    println!("{table}");
}
