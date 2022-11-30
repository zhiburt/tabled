use papergrid::{
    height::HeightEstimator,
    records::vec_records::VecRecords,
    width::{CfgWidthFunction, WidthEstimator},
    AlignmentHorizontal, Borders,
    Entity::Global,
    Estimate, Grid, GridConfig, Indent, Padding,
};

const STYLE: Borders = Borders {
    top: Some('-'),
    top_left: Some('+'),
    top_right: Some('+'),
    top_intersection: Some('+'),
    bottom: Some('-'),
    bottom_left: Some('+'),
    bottom_right: Some('+'),
    bottom_intersection: Some('+'),
    horizontal: Some('-'),
    horizontal_left: Some('+'),
    horizontal_right: Some('+'),
    vertical: Some('|'),
    vertical_left: Some('|'),
    vertical_right: Some('|'),
    intersection: Some('+'),
};

fn main() {
    let mut cfg = GridConfig::default();
    cfg.set_borders(STYLE);
    cfg.set_column_span((1, 1), 3);
    cfg.set_row_span((0, 0), 2);
    cfg.set_alignment_horizontal((1, 0).into(), AlignmentHorizontal::Center);
    cfg.set_alignment_vertical(Global, papergrid::AlignmentVertical::Center);
    cfg.set_padding(
        (0, 0).into(),
        Padding::new(
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

    let records = VecRecords::new(data, (2, 4), CfgWidthFunction::from_cfg(&cfg));

    let mut width = WidthEstimator::default();
    width.estimate(&records, &cfg);

    let mut height = HeightEstimator::default();
    height.estimate(&records, &cfg);

    let grid = Grid::new(&records, &cfg, &width, &height);

    println!("{}", grid);
}
