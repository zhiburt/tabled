use crate::{
    grid::config::SpannedConfig,
    grid::dimension::SpannedGridDimension,
    grid::records::{IntoRecords, Records},
};

pub(crate) fn get_table_widths<R>(records: R, cfg: &SpannedConfig) -> Vec<usize>
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    SpannedGridDimension::width(records, cfg)
}

pub(crate) fn get_table_widths_with_total<R>(records: R, cfg: &SpannedConfig) -> (Vec<usize>, usize)
where
    R: Records,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let widths = SpannedGridDimension::width(records, cfg);
    let total_width = get_table_total_width(&widths, cfg);
    (widths, total_width)
}

fn get_table_total_width(list: &[usize], cfg: &SpannedConfig) -> usize {
    let margin = cfg.get_margin();
    list.iter().sum::<usize>()
        + cfg.count_vertical(list.len())
        + margin.left.size
        + margin.right.size
}
