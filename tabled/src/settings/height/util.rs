use crate::{
    grid::iterable::{ExactDimension, SpannedConfig},
    records::{ExactRecords, Records},
};

pub(crate) fn get_table_height<R: Records + ExactRecords>(
    records: R,
    cfg: &SpannedConfig,
) -> (usize, Vec<usize>) {
    let count_horizontals = cfg.count_horizontal(records.count_rows());

    let margin = cfg.get_margin();
    let margin_size = margin.top.indent.size + margin.bottom.indent.size;

    let list = ExactDimension::height(records, cfg);
    let total = list.iter().sum::<usize>();

    let total = total + count_horizontals + margin_size;

    (total, list)
}
