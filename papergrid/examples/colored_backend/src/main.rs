use owo_colors::{
    colors::{Black, Blue, Red},
    Style as OStyle,
};

use papergrid::{
    height::HeightEstimator, records::records_info_colored::RecordsInfo, width::WidthEstimator,
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
        vertical_intersection: Some('|'),
        horizontal: Some('-'),
        ..Default::default()
    });
    cfg.set_borders_missing('+');

    let records = vec![
        vec![("Hello", Style::default()), ("World", Style::default())],
        vec![("Hi", Style::default()), ("World", Style::default())],
    ];
    let mut records = RecordsInfo::new(records, (3, 5), &cfg);
    records[(0, 0)] = Style(OStyle::default().bg::<Red>().fg::<Black>());
    records[(1, 1)] = Style(OStyle::default().bg::<Blue>());

    let mut width = WidthEstimator::default();
    width.estimate(&records, &cfg);

    let mut height = HeightEstimator::default();
    height.estimate(&records, &cfg);

    let grid = Grid::new(&records, &cfg, width, height);

    println!("{}", grid);
}
