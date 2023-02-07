use crate::grid::config::{Entity, GridConfig};

use crate::{CellOption, TableOption};

#[derive(Debug)]
pub struct FormatConfig<F>(pub F)
where
    F: FnMut(&mut GridConfig);

impl<F, R, D> TableOption<R, D> for FormatConfig<F>
where
    F: FnMut(&mut GridConfig),
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, dimension: &mut D) {
        (self.0)(cfg);
    }
}
