//! This example demonstrates creating a nested [`Table`] by instantiating a new
//! [`Table`] from a collection of other [`Tables`](Table).
//!
//! * This third nested [`Table`] example showcases the [`Table::new()`] approach.

use tabled::{
    settings::{
        object::{Cell, Segment},
        style::Style,
        Alignment, Border, Extract, Highlight, Modify, Panel,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Contribution {
    author: &'static str,
    profile: &'static str,
}

impl Contribution {
    fn new(author: &'static str, profile: &'static str) -> Self {
        Self { author, profile }
    }
}

fn main() {
    let committers = [
        Contribution::new("kozmod", "https:/github.com/kozmod"),
        Contribution::new("IsaacCloos", "https:/github.com/IsaacCloos"),
    ];

    let issuers = [Contribution::new(
        "aharpervc",
        "https:/github.com/aharpervc",
    )];

    let committers_table = Table::new(committers)
        .with(Panel::header("Contributors"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .to_string();

    let issues_table = Table::new(issuers)
        .with(Panel::header("Issuers"))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .to_string();

    let mut a_welcome_table =
        Table::new([String::from("Thank You"), committers_table, issues_table]);
    a_welcome_table
        .with(Extract::rows(1..))
        .with(Style::ascii().remove_horizontal())
        .with(Alignment::center())
        .with(Highlight::border(Cell::new(0, 0), Border::filled('*')));

    println!("{a_welcome_table}");
}
