//! The example can be run by this command
//! `cargo run --example basic`

use tabled::{
    formatting_settings::{AlignmentStrategy, TrimStrategy},
    object::Full,
    Alignment, AlignmentHorizontal, Modify, Style, Table,
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

    let data = [
        [some_json, "2", "3"],
        ["1                  ", some_json, "3"],
        ["1", "2               ", some_json],
    ];

    let table = Table::new(&data).with(Style::modern()).with(
        Modify::new(Full)
            .with(Alignment::Horizontal(AlignmentHorizontal::Right))
            .with(Alignment::center_vertical())
            .with(AlignmentStrategy::PerCell)
            .with(TrimStrategy::None),
    );

    println!("{}", table);
}
