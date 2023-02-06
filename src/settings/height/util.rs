use papergrid::{grid_projection::GridProjection, records::Records, ExactDimension, GridConfig};

use crate::records::ExactRecords;

pub(crate) fn get_table_height<R: Records + ExactRecords>(
    records: R,
    cfg: &GridConfig,
) -> (usize, Vec<usize>) {
    let gp = GridProjection::with_shape(cfg, (records.count_rows(), records.count_columns()));
    let count_horizontals = gp.count_horizontal();

    let margin = cfg.get_margin();
    let margin_size = margin.top.size + margin.bottom.size;

    let list = ExactDimension::height(records, cfg);
    let total = list.iter().sum::<usize>();

    let total = total + count_horizontals + margin_size;

    (total, list)
}
