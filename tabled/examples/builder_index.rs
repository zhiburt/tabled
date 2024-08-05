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

use tabled::{
    settings::{object::Columns, Alignment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct Post {
    title: String,
    #[tabled(format("{} @{}", self.writer, self.team.as_deref().unwrap_or("")))]
    writer: String,
    #[tabled(skip)]
    team: Option<String>,
}

impl Post {
    fn new(title: &str, writer: &str, team: Option<&str>) -> Self {
        Self {
            title: title.to_string(),
            writer: writer.to_string(),
            team: team.map(ToString::to_string),
        }
    }
}

fn main() {
    let content = vec![
        Post::new(
            "crates.io: development update",
            "Tobias Bieniek",
            Some("crates.io"),
        ),
        Post::new("Announcing Rust 1.80.0", "", Some("The Rust Release Team")),
        Post::new("Types Team Update and Roadmap", "lcnr", None),
    ];

    let mut table = Table::new(content);
    table.with(Style::rounded());
    table.modify(Columns::last(), Alignment::right());

    println!("{table}");
}
