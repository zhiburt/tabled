use papergrid::{
    colors::NoColors,
    config::{spanned::SpannedConfig, Borders, Entity, Indent, Position, Sides},
    dimension::{iterable::IterGridDimension, Estimate},
    grid::iterable::IterGrid,
    records::IterRecords,
};

fn main() {
    let data = vec![
        vec!["Papergrid", "is a library", "for printing tables", "!"],
        vec!["", "Just\nlike\nthis", "", ""],
    ];
    let records = IterRecords::new(data, 4, Some(2));

    let cfg = create_config();

    let mut dim = IterGridDimension::default();
    dim.estimate(&records, &cfg);

    let grid = IterGrid::new(records, &cfg, &dim, NoColors);

    println!("{}", grid.to_string());
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
    cfg.set_padding(
        Entity::Global,
        Sides::new(
            Indent::spaced(2),
            Indent::spaced(2),
            Indent::spaced(0),
            Indent::spaced(0),
        ),
    );

    cfg
}
