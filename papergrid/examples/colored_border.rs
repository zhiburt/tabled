use papergrid::{
    ansi::ANSIBuf,
    colors::NoColors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity,
        Indent, Position, Sides,
    },
    dimension::{iterable::IterGridDimension, Estimate},
    grid::iterable::IterGrid,
    records::IterRecords,
};

fn main() {
    let data = vec![
        vec!["Papergrid", "is a library", "for printing tables", "!"],
        vec!["", "Just like this", "", ""],
    ];

    let records = IterRecords::new(data, 4, Some(2));

    let cfg = create_config();

    let mut dim = IterGridDimension::default();
    dim.estimate(&records, &cfg);

    let grid = IterGrid::new(records, &cfg, &dim, NoColors).to_string();

    println!("{grid}");
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
    cfg.set_alignment_horizontal(Entity::Cell(1, 0), AlignmentHorizontal::Center);
    cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
    cfg.set_padding(
        Entity::Cell(0, 0),
        Sides::new(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );
    cfg.set_border_color_default(ANSIBuf::new("\u{1b}[42m", "\u{1b}[0m"));
    cfg.set_border_color(
        Position::new(0, 0),
        Border {
            top: Some(ANSIBuf::new("\u{1b}[43m", "\u{1b}[0m")),
            ..Default::default()
        },
    );
    cfg.set_borders_color(Borders {
        left_intersection: Some(ANSIBuf::new("\u{1b}[43m", "\u{1b}[0m")),
        ..Default::default()
    });

    cfg
}
