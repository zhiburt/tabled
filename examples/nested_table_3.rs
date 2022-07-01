//! The example can be run by this command
//! `cargo run --example nested_table_3`

use tabled::{
    object::{Cell, Segment},
    style::{Border, Style},
    Alignment, Extract, Header, Highlight, Modify, TableIteratorExt, Tabled,
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
    let commiters = [
        Contribution::new("kozmod", "https:/github.com/kozmod"),
        Contribution::new("IsaacCloos", "https:/github.com/IsaacCloos"),
    ];

    let issuers = [Contribution::new(
        "aharpervc",
        "https:/github.com/aharpervc",
    )];

    let commiters_table = commiters
        .table()
        .with(Header("Contributors"))
        .with(Modify::new(Segment::all()).with(Alignment::center()));

    let issues_table = issuers
        .table()
        .with(Header("Issuers"))
        .with(Modify::new(Segment::all()).with(Alignment::center()));

    let a_welcome_table = [
        "Thank You".to_owned(),
        commiters_table.to_string(),
        issues_table.to_string(),
    ]
    .table()
    .with(Extract::rows(1..))
    .with(Style::ascii().horizontal_off())
    .with(Highlight::new(Cell(0, 0), Border::filled('*')));

    println!("{}", a_welcome_table);
}
