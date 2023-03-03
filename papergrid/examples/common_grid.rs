//! The example can be run by this command
//! `cargo run --example common_grid`

use papergrid::{
    config::{AlignmentHorizontal, Borders, Indent, Sides},
    dimension::Estimate,
    grid::compact::{CompactConfig, CompactGrid, ExactDimension},
    records::IterRecords,
};

fn main() {
    let cfg = generate_table_config();

    let data = [
        ["Papergrid", "is a library", "for print tables", "!"],
        [
            "Just like this",
            "NOTICE",
            "that multiline is not supported",
            "H\ne\nl\nl\no",
        ],
    ];
    let records = IterRecords::new(data, 4, None);

    let mut dim = ExactDimension::default();
    dim.estimate(records, &cfg);

    let grid = CompactGrid::new(records, &dim, &cfg);

    println!("{grid}");
}

fn generate_table_config() -> CompactConfig {
    const STYLE: Borders<char> = Borders {
        top: Some('-'),
        top_left: Some('+'),
        top_right: Some('+'),
        top_intersection: Some('+'),
        bottom: Some('-'),
        bottom_left: Some('+'),
        bottom_right: Some('+'),
        bottom_intersection: Some('+'),
        horizontal: Some('-'),
        left_intersection: Some('+'),
        right_intersection: Some('+'),
        vertical: Some('|'),
        left: Some('|'),
        right: Some('|'),
        intersection: Some('+'),
    };

    CompactConfig::default()
        .set_borders(STYLE)
        .set_alignment_horizontal(AlignmentHorizontal::Center)
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(0),
            Indent::spaced(0),
        ))
}
