use crate::grid::config::{Entity, GridConfig};

// todo: Update documentation
/// A trait for configuring a single cell.
/// 
/// ~~~~ Where cell represented by 'row' and 'column' indexes. ~~~~
///
/// A cell can be targeted by [`Cell`].
///
/// [`Cell`]: crate::object::Cell
pub trait CellOption<R> {
    /// Modification function of a single cell.
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity);
}

impl<T, R> CellOption<R> for &mut T
where
    T: CellOption<R> + ?Sized,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        T::change(self, records, cfg, entity);
    }
}
