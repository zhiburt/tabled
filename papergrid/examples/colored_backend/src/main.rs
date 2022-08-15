use owo_colors::{
    colors::{Black, Blue, Red},
    Style as OStyle,
};

use papergrid::{
    height::HeightEstimator,
    records::{cell_info::CellInfo, tcell::TCell, vec_records::VecRecords},
    width::{CfgWidthFunction, WidthEstimator},
    Borders, Color, Estimate, Grid, GridConfig,
};

#[derive(Debug, Clone, Default)]
struct Style(OStyle);

impl Color for Style {
    fn fmt_prefix(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_prefix(f)
    }
}

fn main() {
    let mut cfg = GridConfig::default();
    cfg.set_borders(Borders {
        top: Some('-'),
        bottom: Some('-'),
        vertical_left: Some('|'),
        vertical_right: Some('|'),
        vertical: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });
    cfg.set_borders_missing('+');

    let width_ctrl = CfgWidthFunction::from_cfg(&cfg);

    let records = vec![
        vec![
            TCell::from(CellInfo::new("Hello", &width_ctrl)),
            TCell::from(CellInfo::new("World", &width_ctrl)),
        ],
        vec![
            TCell::from(CellInfo::new("Hi", &width_ctrl)),
            TCell::from(CellInfo::new("World", &width_ctrl)),
        ],
    ];

    let mut records = VecRecords::from(records);
    *records[(0, 0)].get_data_mut() = Style(OStyle::default().bg::<Red>().fg::<Black>());
    *records[(1, 1)].get_data_mut() = Style(OStyle::default().bg::<Blue>());

    let mut width = WidthEstimator::default();
    width.estimate(&records, &cfg);

    let mut height = HeightEstimator::default();
    height.estimate(&records, &cfg);

    let grid = Grid::new(&records, &cfg, &width, &height);

    println!("{}", grid);
}
