//! The example can be run by this command
//! `cargo run --example table_enclosure`

use papergrid::Border;
use tabled::{
    object::{Cell, Segment},
    Alignment, Extract, Header, Highlight, Modify, Style, TableIteratorExt, Tabled,
};

#[derive(Tabled)]
struct Contribution {
    author: &'static str,
    profile: &'static str,
}

fn main() {
    let commiters = [
        Contribution {
            author: "kozmod",
            profile: "https:/github.com/kozmod",
        },
        Contribution {
            author: "IsaacCloos",
            profile: "https:/github.com/IsaacCloos",
        },
    ];

    let issuers = [Contribution {
        author: "aharpervc",
        profile: "https:/github.com/aharpervc",
    }];

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
