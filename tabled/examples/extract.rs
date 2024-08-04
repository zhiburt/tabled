//! This example demonstrates using the [`Extract`] [`TableOption`] to
//! produce a subsection of a [`Table`].
//!
//! * [`Extract`] can return a new [`Table`] with three functions:
//!     * `rows()` | yields subset of the initial rows
//!     * `columns()` | yields subset of the initial columns
//!     * `segment()` | yields subsection of the initial table
//!
//! * Note how [`Extract`] methods accepts [`RangeBounds`] arguments,
//!   making subset specifications concise.

use tabled::{
    settings::{object::Rows, Alignment, Extract, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Album {
    artist: String,
    name: String,
    released: String,
    #[tabled(format = "{:?}")]
    greatness: Greatness,
}

impl Album {
    fn new(artist: &str, name: &str, released: &str, greatness: Greatness) -> Self {
        Self {
            name: name.to_string(),
            artist: artist.to_string(),
            released: released.to_string(),
            greatness,
        }
    }
}

#[derive(Debug)]
enum Greatness {
    Supreme,
    Outstanding,
    Unparalleled,
}

fn main() {
    use Greatness::*;

    #[rustfmt::skip]
    let data = [
        Album::new("Pink Floyd", "The Dark Side of the Moon", "01 March 1973", Unparalleled),
        Album::new("Fleetwood Mac", "Rumours", "04 February 1977", Outstanding),
        Album::new("Led Zeppelin", "Led Zeppelin IV", "08 November 1971", Supreme),
    ];

    println!("Full table");

    let mut table = Table::new(data);
    table
        .with(Style::modern())
        .modify(Rows::first(), Alignment::center())
        .modify(Rows::new(1..), Alignment::left());
    println!("{table}");

    println!("Segment   row: (1..=2)   column: (1..)");

    table.with(Extract::segment(1..=2, 1..));
    println!("{table}");
}
