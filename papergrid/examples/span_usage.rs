use papergrid::{
    colors::NoColors,
    config::{
        spanned::SpannedConfig, AlignmentHorizontal, AlignmentVertical, Borders, Entity, Indent,
        Sides,
    },
    dimension::{spanned::SpannedGridDimension, Estimate},
    grid::peekable::PeekableGrid,
    records::vec_records::{CellInfo, VecRecords},
};

fn main() {
    let mut cfg = SpannedConfig::default();
    cfg.set_borders(Borders {
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
    });
    cfg.set_column_span((1, 1), 3);
    cfg.set_row_span((0, 0), 2);
    cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
    cfg.set_alignment_vertical(Entity::Global, AlignmentVertical::Center);
    cfg.set_padding(
        (0, 0).into(),
        Sides::new(
            Indent::spaced(4),
            Indent::spaced(4),
            Indent::spaced(1),
            Indent::spaced(1),
        ),
    );

    let data = [
        ["Papergrid", "is a library", "for print tables", "!"],
        ["", "Just like this", "", ""],
    ];

    let data = data
        .iter()
        .map(|row| row.iter().map(CellInfo::new).collect())
        .collect();

    let records = VecRecords::new(data);

    let mut dims = SpannedGridDimension::default();
    dims.estimate(&records, &cfg);

    let grid = PeekableGrid::new(&records, &cfg, &dims, NoColors).to_string();

    println!("{grid}");
}
