use papergrid::{
    colors::NoColors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Borders, Entity::Global,
        Indent, Position, Sides,
    },
    dimension::{iterable::IterGridDimension, Estimate},
    grid::iterable::IterGrid,
    records::IterRecords,
};

fn main() {
    let data = [
        ["Papergrid", "is a library", "for printing tables", "!"],
        [
            "",
            "Just                                         like\n\nthis",
            "",
            "",
        ],
    ];

    let records = IterRecords::new(data, 4, None);

    let cfg = generate_table_config();

    let mut dim = IterGridDimension::default();
    dim.estimate(records, &cfg);

    let grid = IterGrid::new(records, &cfg, &dim, NoColors);
    let grid = grid.to_string();

    println!("{grid}");
}

fn generate_table_config() -> SpannedConfig {
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

    let mut cfg = SpannedConfig::default();
    cfg.set_borders(STYLE);
    cfg.set_column_span(Position::new(1, 1), 3);
    cfg.set_row_span(Position::new(0, 0), 2);
    cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
    cfg.set_alignment_vertical(Global, AlignmentVertical::Center);
    cfg.set_padding(
        (0, 0).into(),
        Sides::new(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );

    cfg
}
