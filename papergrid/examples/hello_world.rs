use papergrid::{
    config::{AlignmentHorizontal, AlignmentVertical, Borders, Entity::Global, Indent, Sides},
    dimension::Estimate,
    grid::spanned::{dimension::ExactDimension, Grid, GridConfig},
    records::IterRecords,
};

fn main() {
    let cfg = generate_table_config();

    let data = [
        ["Papergrid", "is a library", "for print tables", "!"],
        ["", "Just like this", "", ""],
    ];
    let records = IterRecords::new(data, 4, None);

    let mut dim = ExactDimension::default();
    dim.estimate(records, &cfg);

    let grid = Grid::new(records, &dim, &cfg).to_string();

    println!("{grid}");
}

fn generate_table_config() -> GridConfig {
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

    let mut cfg = GridConfig::default();
    cfg.set_borders(STYLE);
    cfg.set_column_span((1, 1), 3);
    cfg.set_row_span((0, 0), 2);
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
