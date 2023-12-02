use crate::grid::{
    config::SpannedConfig,
    dimension::SpannedGridDimension,
    records::{ExactRecords, IntoRecords, Records},
};

pub(crate) fn get_table_height<R>(records: R, cfg: &SpannedConfig) -> (usize, Vec<usize>)
where
    R: Records + ExactRecords,
    <R::Iter as IntoRecords>::Cell: AsRef<str>,
{
    let count_horizontals = cfg.count_horizontal(records.count_rows());

    let margin = cfg.get_margin();
    let margin_size = margin.top.size + margin.bottom.size;

    let list = SpannedGridDimension::height(records, cfg);
    let total = list.iter().sum::<usize>();

    let total = total + count_horizontals + margin_size;

    (total, list)
}
