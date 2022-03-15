//! The example can be run by this command
//! `cargo run --example extract`

use std::fmt::{Display, Formatter};

use tabled::{
    Alignment, AlignmentHorizontal, Column, Extract, Format, Head, Modify, Row, Style, Table,
    Tabled,
};

#[derive(Tabled)]
struct Album {
    artist: &'static str,
    name: &'static str,
    released: &'static str,
    level_of_greatness: LevelOfGreatness,
}

enum LevelOfGreatness {
    Supreme,
    Outstanding,
    Unparalleled,
}

impl Display for LevelOfGreatness {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LevelOfGreatness::Supreme => write!(f, "Supreme"),
            LevelOfGreatness::Outstanding => write!(f, "Outstanding"),
            LevelOfGreatness::Unparalleled => write!(f, "Unparalleled"),
        }
    }
}

fn main() {
    let data = [
        Album {
            artist: "Pink Floyd",
            name: "The Dark Side of the Moon",
            released: "01 March 1973",
            level_of_greatness: LevelOfGreatness::Unparalleled,
        },
        Album {
            artist: "Fleetwood Mac",
            name: "Rumours",
            released: "04 February 1977",
            level_of_greatness: LevelOfGreatness::Outstanding,
        },
        Album {
            artist: "Led Zeppelin",
            name: "Led Zeppelin IV",
            released: "08 November 1971",
            level_of_greatness: LevelOfGreatness::Supreme,
        },
    ];

    println!("Full");
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Modify::new(Head).with(Alignment::Horizontal(AlignmentHorizontal::Center)))
        .with(Modify::new(Row(1..)).with(Alignment::Horizontal(AlignmentHorizontal::Left)));
    println!("{}", table);

    println!("Segment   row: (1..=2)   column: (1..)");
    let table = table.with(Extract::new(1..=2, 1..));
    println!("{}", table);

    println!("Refinished segment");
    let table = table
        .with(Style::modern())
        .with(Modify::new(Column(1..)).with(Format(|s| {
            if s == "Outstanding" {
                format!("+{}+", s)
            } else {
                format!("{}", s)
            }
        })));
    println!("{}", table);
}
