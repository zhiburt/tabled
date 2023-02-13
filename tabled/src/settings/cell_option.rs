use papergrid::records::Records;

use crate::{
    grid::config::Entity,
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
pub trait CellOption<R, C> {
    /// Modification function of a single cell.
    fn change(&mut self, records: &mut R, cfg: &mut C, entity: Entity);
}

impl<T, R, C> CellOption<R, C> for &mut T
where
    T: CellOption<R, C> + ?Sized,
{
    fn change(&mut self, records: &mut R, cfg: &mut C, entity: Entity) {
        T::change(self, records, cfg, entity);
    }
}

impl<R, C> CellOption<R, C> for String
where
    R: Records + ExactRecords + RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            records.set(pos, self.clone());
        }
    }
}

impl<'a, R, C> CellOption<R, C> for &'a str
where
    R: Records + ExactRecords + RecordsMut<&'a str>,
{
    fn change(&mut self, records: &mut R, _: &mut C, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            records.set(pos, self);
        }
    }
}
