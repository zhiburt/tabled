//! This example demonstrates the flexibility of [`papergrid`] with manual configurations
//! of [`Borders`], [`CompactConfig`], and column counts with [`IterRecords`].
//!
//! * For an alternative to [`CompactGrid`] and [`CompactGridDimension`] with
//! flexible row height, variable intra-column spans, and multiline cell support
//! see [`Grid`] and [`SpannedGridDimension`].

use papergrid::{
    config::compact::CompactConfig,
    config::{AlignmentHorizontal, Borders, Indent, Sides},
    dimension::compact::CompactGridDimension,
    dimension::Estimate,
    grid::compact::CompactGrid,
    records::IterRecords,
};

fn main() {
    let cfg = generate_table_config();

    let data = [
        ["Papergrid", "is a library", "for printing tables", "!"],
        [
            "Just like this",
            "NOTICE",
            "that multiline is not supported",
            "H\ne\nl\nl\no",
        ],
    ];
    let records = IterRecords::new(data, 4, None);

    let mut dim = CompactGridDimension::default();
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
