use papergrid::{
    colors::NoColors,
    config::{compact::CompactConfig, AlignmentHorizontal, Borders, Indent, Sides},
    dimension::{compact::CompactGridDimension, Estimate},
    grid::compact::CompactGrid,
    records::IterRecords,
};

fn main() {
    let data = [
        ["Papergrid", "is a library", "for printing tables", "!"],
        ["Just like this", "", "", "!"],
        ["NOTICE", "that multiline is not supported", "N\nO\n", "!"],
    ];

    let records = IterRecords::new(data, 4, None);

    let cfg = generate_table_config();

    let mut dim = CompactGridDimension::default();
    dim.estimate(records, &cfg);

    let grid = CompactGrid::new(records, &cfg, &dim, NoColors);

    println!("{grid}");
}

const fn generate_table_config() -> CompactConfig {
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

    CompactConfig::new()
        .set_borders(STYLE)
        .set_alignment_horizontal(AlignmentHorizontal::Center)
        .set_padding(Sides::new(
            Indent::spaced(1),
            Indent::spaced(1),
            Indent::spaced(0),
            Indent::spaced(0),
        ))
}
