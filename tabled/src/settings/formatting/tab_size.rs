use crate::{grid::config::GridConfig, settings::TableOption};

/// Set a tab size.
///
/// The size is used in order to calculate width correctly.
///
/// Default value is 4 (basically 1 '\t' equals 4 spaces).
///
/// IMPORTANT: The tab character might be not present in output,
/// it might be replaced by spaces.
#[derive(Debug, Default, Clone)]
pub struct TabSize(pub usize);

impl<R, D> TableOption<R, D> for TabSize {
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        cfg.set_tab_width(self.0);
    }
}
