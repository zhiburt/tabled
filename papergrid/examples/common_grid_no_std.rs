//! This example demonstrates using [`papergrid`] without [The Rust Standard Library](std).
//!
//! * Note the missing, pre-built [`Dimension`] implementations requiring manual design.

use papergrid::{
    config::compact::CompactConfig,
    config::{AlignmentHorizontal, Borders, Indent, Sides},
    dimension::Dimension,
    grid::compact::CompactGrid,
    records::IterRecords,
};

fn main() {
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
    let dim = ConstDims(&[20, 15, 40, 3], 4);
    let cfg = generate_table_config();

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
            Indent::spaced(3),
            Indent::spaced(0),
        ))
}

struct ConstDims<'a>(&'a [usize], usize);

impl Dimension for ConstDims<'_> {
    fn get_width(&self, column: usize) -> usize {
        self.0[column]
    }

    fn get_height(&self, _: usize) -> usize {
        self.1
    }
}
