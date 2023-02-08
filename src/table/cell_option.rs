use papergrid::records::Records;

use crate::{
    grid::config::{Entity, GridConfig},
    records::{ExactRecords, RecordsMut},
};

// todo: Update documentation
// todo: Move to settings

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

impl<R> CellOption<R> for String
where
    R: Records + ExactRecords + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, _: &mut GridConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            records.set(pos, self.clone());
        }
    }
}

impl<R> CellOption<R> for &str
where
    R: Records + ExactRecords + RecordsMut<Text = String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: Entity) {
        let mut string = self.to_owned();
        <String as CellOption<R>>::change(&mut string, records, cfg, entity);
    }
}
