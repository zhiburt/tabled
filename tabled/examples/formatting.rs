use std::iter::FromIterator;

use tabled::{
    col, row,
    settings::{
        formatting::{AlignmentStrategy, TrimStrategy},
        Alignment, Style,
    },
    Table,
};

fn main() {
    let some_json = r#"[
    "foo",
    {
        "bar": 1,
        "baz": [
            2,
            3
        ]
    }
]"#;

    let mut origin = Table::from_iter([[some_json]]);
    origin
        .with(Style::rounded().remove_horizontals())
        .with(Alignment::center());

    let t1 = origin.clone();

    origin.with(AlignmentStrategy::PerLine);
    let t2 = origin.clone();

    origin
        .with(AlignmentStrategy::PerCell)
        .with(TrimStrategy::Both);
    let t3 = origin;

    let mut output = row![
        col!["Per cell Alignment strategy + no trim", t1].with(Style::empty()),
        col!["Per line Alignment strategy", t2].with(Style::empty()),
        col!["Per cell Alignment strategy + trim", t3].with(Style::empty())
    ];
    output.with(Style::blank().vertical('│'));

    println!("{output}");
}
