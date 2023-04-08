//! This example demonstrates using the [`Alignment`], [`AlignmentStrategy`], and [`TrimStrategy`] [`CellOptions`]
//! to align the content of a [`Table`] in several nuanced ways.
//!
//! * Note how [`AlignmentStrategy`] and [`TrimStrategy`] provide useful tools for managing multiline cells and
//! cell values that are bloated with whitespace.

use tabled::{
    settings::{
        formatting::{AlignmentStrategy, TrimStrategy},
        object::Segment,
        Alignment, Modify, Style,
    },
    Table,
};

fn main() {
    let some_json = r#"
[
    "foo",
    {
        "bar": 1,
        "baz": [
            2,
            3
        ]
    }
]"#;

    let mut table = Table::new([some_json]);
    table
        .with(Style::rounded())
        .with(Modify::new(Segment::all()).with(Alignment::center()));

    println!("A default Alignment settings\n{table}");

    table.with(Modify::new(Segment::all()).with(AlignmentStrategy::PerLine));

    println!("Per line Alignment strategy\n{table}");

    table.with(
        Modify::new(Segment::all())
            .with(AlignmentStrategy::PerCell)
            .with(TrimStrategy::Both),
    );

    println!("A default Alignment; allowing vertical and horizontal trim\n{table}");
}
