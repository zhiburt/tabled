//! This example demonstrates using the [`Extract`] [`TableOption`] to
//! produce a subsection of a [`Table`].
//!
//! * [`Extract`] can return a new [`Table`] with three functions:
//!     * `rows()` | yields subset of the initial rows
//!     * `columns()` | yields subset of the initial columns
//!     * `segment()` | yields subsection of the initial table
//!
//! * Note how [`Extract`] methods accepts [`RangeBounds`] arguments,
//! making subset specifications concise.

use std::fmt::{Display, Formatter};

use tabled::{
    settings::{
        object::{Columns, Rows},
        Alignment, Extract, Format, Modify, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Album {
    artist: &'static str,
    name: &'static str,
    released: &'static str,
    level_of_greatness: LevelOfGreatness,
}

impl Album {
    fn new(
        artist: &'static str,
        name: &'static str,
        released: &'static str,
        level_of_greatness: LevelOfGreatness,
    ) -> Self {
        Self {
            artist,
            name,
            released,
            level_of_greatness,
        }
    }
}

#[derive(Debug)]
enum LevelOfGreatness {
    Supreme,
    Outstanding,
    Unparalleled,
}

impl Display for LevelOfGreatness {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self, f)
    }
}

fn main() {
    use LevelOfGreatness::*;

    let data = [
        Album::new(
            "Pink Floyd",
            "The Dark Side of the Moon",
            "01 March 1973",
            Unparalleled,
        ),
        Album::new("Fleetwood Mac", "Rumours", "04 February 1977", Outstanding),
        Album::new(
            "Led Zeppelin",
            "Led Zeppelin IV",
            "08 November 1971",
            Supreme,
        ),
    ];

    println!("Full");

    let mut table = Table::new(data);
    table
        .with(Style::modern())
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));
    println!("{table}");

    println!("Segment   row: (1..=2)   column: (1..)");

    let table = table.with(Extract::segment(1..=2, 1..));
    println!("{table}");

    println!("Refinished segment");

    let highlight = Format::content(|s| {
        if s == "Outstanding" {
            format!("+{s}+")
        } else {
            s.to_string()
        }
    });

    let table = table
        .with(Style::modern())
        .with(Modify::new(Columns::new(1..)).with(highlight));
    println!("{table}");
}
