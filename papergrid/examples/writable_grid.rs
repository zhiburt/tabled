use papergrid::{
    colors::NoColors,
    config::{spanned::SpannedConfig, AlignmentVertical, Borders, Entity, Indent, Position, Sides},
    dimension::{peekable::PeekableGridDimension, Estimate},
    grid::writable::WritableGrid,
    records::vec_records::VecRecords,
};

fn main() {
    let data = vec![
        vec!["Papergrid", "is a library", "for printing tables", "!"],
        vec!["", "Just like this", "", ""],
    ];
    let records = VecRecords::new(data);

    let cfg = create_config();

    let mut dim = PeekableGridDimension::default();
    dim.estimate(&records, &cfg);

    let grid = WritableGrid::new(&records, &cfg, &dim, NoColors);

    let mut buf = String::new();
    grid.build(&mut buf).unwrap();

    println!("{buf}");
}

fn create_config() -> SpannedConfig {
    let style = Borders {
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

    let mut cfg = SpannedConfig::new();
    cfg.set_borders(style);
    cfg.set_column_span(Position::new(1, 1), 3);
    cfg.set_row_span(Position::new(0, 0), 2);
    cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
    cfg.set_padding(
        Entity::Cell(0, 0),
        Sides::new(
            Indent::zero(),
            Indent::zero(),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );

    cfg
}
