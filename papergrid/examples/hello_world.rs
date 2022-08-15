use papergrid::{
    height::HeightEstimator,
    records::vec_records::VecRecords,
    width::{CfgWidthFunction, WidthEstimator},
    Borders, Estimate, Grid, GridConfig,
};

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

    let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
    let records = VecRecords::new(&records, (3, 5), CfgWidthFunction::from_cfg(&cfg));

    let mut width = WidthEstimator::default();
    width.estimate(&records, &cfg);

    let mut height = HeightEstimator::default();
    height.estimate(&records, &cfg);

    let grid = Grid::new(&records, &cfg, &width, &height);

    println!("{}", grid);
}
