//! This example demonstrates creating a nested [`Table`] by instantiating a new
//! [`Table`] from a collection of other [`Tables`](Table).
//!
//! * This third nested [`Table`] example showcases the [`Table::new()`] approach.

use tabled::{
    settings::{
        object::Rows,
        style::{BorderSpanCorrection, Style},
        Alignment, Extract, Highlight, Padding, Panel,
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
        .with(Alignment::center())
        .with(BorderSpanCorrection)
        .to_string();

    let issues_table = Table::new(issuers)
        .with(Panel::header("Issuers"))
        .with(Alignment::center())
        .with(BorderSpanCorrection)
        .to_string();

    let mut welcome_table = Table::new([(committers_table, issues_table)]);
    welcome_table
        .with(Extract::rows(1..))
        .with(Panel::header("Thank You"))
        .with(Style::ascii().remove_horizontal())
        .modify(Rows::new(1..), Padding::new(1, 1, 1, 0))
        .with(Alignment::center())
        .with(Highlight::outline(Rows::first(), '*'));

    println!("{welcome_table}");
}
