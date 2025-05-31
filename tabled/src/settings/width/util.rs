use crate::grid::config::SpannedConfig;

pub(crate) fn get_table_total_width(list: &[usize], cfg: &SpannedConfig) -> usize {
    let margin = cfg.get_margin();
    list.iter().sum::<usize>()
        + cfg.count_vertical(list.len())
        + margin.left.size
        + margin.right.size
}
