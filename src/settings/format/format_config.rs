use crate::grid::config::{Entity, GridConfig};

use crate::CellOption;

#[derive(Debug)]
pub struct FormatConfig<F>(pub F)
where
    F: FnMut(&mut GridConfig);

impl<F, R> CellOption<R> for FormatConfig<F>
where
    F: FnMut(&mut GridConfig),
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        (self.0)(cfg);
    }
}
