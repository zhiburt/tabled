use crate::{grid::compact::CompactConfig, grid::spanned::GridConfig, settings::TableOption};

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

impl<R, D> TableOption<R, D, GridConfig> for TabSize {
    fn change(&mut self, _: &mut R, cfg: &mut GridConfig, _: &mut D) {
        cfg.set_tab_width(self.0);
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for TabSize {
    fn change(&mut self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_tab_width(self.0);
    }
}
